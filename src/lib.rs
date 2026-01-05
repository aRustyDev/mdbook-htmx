//! mdbook-htmx library
//!
//! Provides the core rendering functionality for the mdbook-htmx backend.

pub mod assets;
pub mod config;
pub mod context;
pub mod error;
pub mod frontmatter;
pub mod manifest;
pub mod render;
pub mod search;
pub mod templates;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use log::{debug, info};
use tera::Tera;

pub use config::{HtmxConfig, OutputMode};
pub use context::{Chapter, RenderContext};
pub use error::BuildError;

use crate::frontmatter::Frontmatter;
use crate::manifest::{Manifest, PageEntry};
use crate::render::oob::{render_oob_updates, NavItem, SidebarContext};

/// The main renderer that processes MDBook content and produces HTMX-enhanced HTML.
pub struct HtmxRenderer {
    /// The parsed render context from MDBook
    ctx: RenderContext,
    /// HTMX-specific configuration
    config: HtmxConfig,
    /// Tera template engine
    tera: Tera,
    /// Output directory
    output_dir: PathBuf,
}

/// Result of rendering a single chapter.
pub struct RenderedChapter {
    /// The full HTML page (with layout)
    pub page: String,
    /// The content-only fragment (for HTMX)
    pub fragment: String,
    /// Parsed frontmatter
    pub frontmatter: Frontmatter,
    /// Output path relative to output directory
    pub path: PathBuf,
}

impl HtmxRenderer {
    /// Create a new renderer from JSON RenderContext.
    ///
    /// # Arguments
    /// * `json` - The JSON string containing the MDBook RenderContext
    ///
    /// # Returns
    /// A configured HtmxRenderer ready to render
    pub fn from_json(json: &str) -> Result<Self> {
        let ctx: RenderContext =
            serde_json::from_str(json).context("Failed to parse RenderContext JSON")?;

        // Check MDBook version
        if !ctx.is_supported_version() {
            anyhow::bail!(
                "MDBook version {} is not supported. Requires 0.4.21+",
                ctx.version
            );
        }

        // Load HTMX config from [output.htmx]
        let htmx_config_value = ctx.config.output.get("htmx");
        let config = HtmxConfig::from_toml(htmx_config_value)?;
        config.validate()?;

        debug!("Loaded config: {:?}", config);

        // Initialize template engine
        let tera = templates::init_templates()?;

        // Determine output directory
        let output_dir = ctx.destination.clone();

        Ok(Self {
            ctx,
            config,
            tera,
            output_dir,
        })
    }

    /// Render the book to the output directory.
    ///
    /// This produces:
    /// - `pages/*.html` - Full HTML pages
    /// - `fragments/*.html` - Content-only fragments for HTMX
    /// - `manifest.json` - Page metadata for server integration
    pub fn render(&self) -> Result<()> {
        info!("Rendering to {}", self.output_dir.display());

        // Create output directories
        self.create_directories()?;

        // Build manifest as we render
        let mut manifest = Manifest::new();

        // Collect all chapters for prev/next navigation
        let chapters: Vec<_> = self.ctx.iter_chapters().collect();
        let chapter_count = chapters.len();

        info!("Rendering {} chapters", chapter_count);

        // Render each chapter
        for (idx, chapter) in chapters.iter().enumerate() {
            // Skip chapters without a path (draft chapters)
            let Some(ref path) = chapter.path else {
                debug!("Skipping draft chapter: {}", chapter.name);
                continue;
            };

            debug!("Rendering chapter: {} ({})", chapter.name, path.display());

            // Get prev/next chapters for navigation
            let prev_chapter = if idx > 0 { chapters.get(idx - 1) } else { None };
            let next_chapter = chapters.get(idx + 1);

            // Render the chapter
            let rendered = self.render_chapter(
                chapter,
                prev_chapter.copied(),
                next_chapter.copied(),
                &chapters,
            )?;

            // Write output files
            self.write_chapter(&rendered)?;

            // Add to manifest
            let url_path = self.path_to_url(path);
            manifest.add_page(
                url_path,
                PageEntry {
                    title: rendered
                        .frontmatter
                        .title
                        .clone()
                        .unwrap_or_else(|| chapter.name.clone()),
                    source: chapter.source_path.clone().unwrap_or_else(|| path.clone()),
                    page_path: PathBuf::from("pages").join(&rendered.path),
                    fragment_path: PathBuf::from("fragments").join(&rendered.path),
                    scope: rendered.frontmatter.scope.clone(),
                    authn: rendered.frontmatter.authn.as_ref().map(|a| a.to_string()),
                    authz: rendered.frontmatter.authz.clone(),
                    fallback: rendered.frontmatter.fallback.clone(),
                    content_hash: assets::compute_short_hash(rendered.page.as_bytes()),
                },
            );
        }

        // Write manifest
        if self.config.search.generate_index {
            self.write_manifest(&manifest)?;
        }

        info!("Rendering complete");
        Ok(())
    }

    /// Create output directory structure.
    fn create_directories(&self) -> Result<()> {
        let dirs = ["pages", "fragments", "assets", "oob"];

        for dir in dirs {
            let path = self.output_dir.join(dir);
            fs::create_dir_all(&path)
                .with_context(|| format!("Failed to create directory: {}", path.display()))?;
        }

        Ok(())
    }

    /// Render a single chapter.
    fn render_chapter(
        &self,
        chapter: &Chapter,
        prev: Option<&Chapter>,
        next: Option<&Chapter>,
        all_chapters: &[&Chapter],
    ) -> Result<RenderedChapter> {
        let path = chapter.path.as_ref().unwrap();

        // Parse frontmatter
        let (frontmatter, content) = frontmatter::parse_frontmatter(&chapter.content, path)?;

        // Convert Markdown to HTML
        let html_content = render::markdown_to_html(content);

        // Build template context
        let mut context = tera::Context::new();

        // Page context
        context.insert(
            "page",
            &serde_json::json!({
                "title": frontmatter.title.as_ref().unwrap_or(&chapter.name),
                "description": frontmatter.description,
                "content": html_content,
                "path": self.path_to_url(path),
                "source_path": chapter.source_path,
                "scopes": frontmatter.scope.as_ref().map(|s| vec![s.clone()]).unwrap_or_default(),
                "meta": {
                    "description": frontmatter.description,
                },
                "htmx": {
                    "lazy": false,
                }
            }),
        );

        // Config context
        context.insert(
            "config",
            &serde_json::json!({
                "book": {
                    "title": self.ctx.config.book.title,
                    "description": self.ctx.config.book.description,
                    "language": self.ctx.config.book.language,
                },
                "htmx": {
                    "boost": self.config.boost,
                    "target": self.config.target,
                    "swap_strategy": self.config.swap_strategy.to_string(),
                    "push_url": self.config.push_url,
                    "navigation": {
                        "breadcrumbs": self.config.navigation.breadcrumbs,
                        "toc": self.config.navigation.toc,
                        "prev_next": self.config.navigation.prev_next,
                    }
                }
            }),
        );

        // Navigation context
        if let Some(prev) = prev {
            if let Some(ref prev_path) = prev.path {
                context.insert(
                    "prev_page",
                    &serde_json::json!({
                        "title": prev.name,
                        "path": self.path_to_url(prev_path),
                    }),
                );
            }
        }

        if let Some(next) = next {
            if let Some(ref next_path) = next.path {
                context.insert(
                    "next_page",
                    &serde_json::json!({
                        "title": next.name,
                        "path": self.path_to_url(next_path),
                    }),
                );
            }
        }

        // Build navigation sidebar context
        let active_path = self.path_to_url(path);
        let nav_items: Vec<NavItem> = all_chapters
            .iter()
            .filter_map(|ch| {
                ch.path.as_ref().map(|p| {
                    let url = self.path_to_url(p);
                    let is_active = url == active_path;
                    NavItem {
                        title: ch.name.clone(),
                        path: url.clone(),
                        is_active,
                        children: vec![],
                        is_expanded: is_active || active_path.starts_with(&url),
                        number: ch.number.as_ref().map(|nums| {
                            nums.iter()
                                .map(|n| n.to_string())
                                .collect::<Vec<_>>()
                                .join(".")
                        }),
                    }
                })
            })
            .collect();

        let sidebar_ctx = SidebarContext {
            items: nav_items,
            active_path: active_path.clone(),
        };
        context.insert("sidebar", &sidebar_ctx);
        context.insert(
            "navigation",
            &serde_json::json!({ "items": sidebar_ctx.items }),
        );

        // Generate OOB updates for fragment
        let oob_updates =
            render_oob_updates(&self.tera, chapter, all_chapters, &active_path).unwrap_or_default();
        context.insert("oob_updates", &oob_updates.to_html());

        // Render full page
        let page = self
            .tera
            .render("docs/page.html", &context)
            .with_context(|| format!("Failed to render page template for {}", path.display()))?;

        // Render fragment
        let fragment = self
            .tera
            .render("docs/fragment.html", &context)
            .with_context(|| {
                format!("Failed to render fragment template for {}", path.display())
            })?;

        // Compute output path (convert .md to .html)
        let output_path = path.with_extension("html");

        Ok(RenderedChapter {
            page,
            fragment,
            frontmatter,
            path: output_path,
        })
    }

    /// Write rendered chapter to output files.
    fn write_chapter(&self, rendered: &RenderedChapter) -> Result<()> {
        // Write full page
        let page_path = self.output_dir.join("pages").join(&rendered.path);
        if let Some(parent) = page_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&page_path, &rendered.page)
            .with_context(|| format!("Failed to write page: {}", page_path.display()))?;

        // Write fragment if configured
        if self.config.output_mode != OutputMode::Full {
            let fragment_path = self.output_dir.join("fragments").join(&rendered.path);
            if let Some(parent) = fragment_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&fragment_path, &rendered.fragment).with_context(|| {
                format!("Failed to write fragment: {}", fragment_path.display())
            })?;
        }

        Ok(())
    }

    /// Write manifest.json.
    fn write_manifest(&self, manifest: &Manifest) -> Result<()> {
        let path = self.output_dir.join("manifest.json");
        let json = manifest.to_json()?;
        fs::write(&path, json)
            .with_context(|| format!("Failed to write manifest: {}", path.display()))?;
        info!("Wrote manifest.json with {} pages", manifest.pages.len());
        Ok(())
    }

    /// Convert file path to URL path.
    fn path_to_url(&self, path: &Path) -> String {
        let path_str = path.with_extension("").to_string_lossy().to_string();

        // Handle index pages
        if path_str == "README" || path_str.ends_with("/README") {
            if path_str == "README" {
                "/".to_string()
            } else {
                format!("/{}/", path_str.trim_end_matches("/README"))
            }
        } else {
            format!("/{}", path_str.replace('\\', "/"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_to_url() {
        let renderer = create_test_renderer();

        assert_eq!(renderer.path_to_url(Path::new("chapter1.md")), "/chapter1");
        assert_eq!(renderer.path_to_url(Path::new("README.md")), "/");
        assert_eq!(
            renderer.path_to_url(Path::new("guide/intro.md")),
            "/guide/intro"
        );
    }

    fn create_test_renderer() -> HtmxRenderer {
        HtmxRenderer {
            ctx: RenderContext {
                version: "0.4.40".to_string(),
                root: PathBuf::new(),
                book: context::Book { sections: vec![] },
                config: context::BookConfig {
                    book: context::BookMetadata {
                        title: Some("Test".to_string()),
                        authors: vec![],
                        description: None,
                        src: PathBuf::from("src"),
                        language: "en".to_string(),
                    },
                    build: context::BuildConfig::default(),
                    output: std::collections::HashMap::new(),
                },
                destination: PathBuf::from("book/htmx"),
            },
            config: HtmxConfig::default(),
            tera: Tera::default(),
            output_dir: PathBuf::from("book/htmx"),
        }
    }
}
