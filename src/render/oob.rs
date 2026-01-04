//! Out-of-Band (OOB) swap generation.
//!
//! Generates OOB updates for sidebar and breadcrumbs that are appended
//! to fragment responses. This enables updating multiple parts of the
//! page with a single HTMX request.

use serde::Serialize;
use tera::{Context, Tera};

use crate::context::Chapter;

/// Breadcrumb entry for navigation trail.
#[derive(Debug, Clone, Serialize)]
pub struct Breadcrumb {
    /// Display title
    pub title: String,
    /// URL path
    pub path: String,
    /// Whether this is the current page
    pub is_current: bool,
}

/// Navigation item for sidebar.
#[derive(Debug, Clone, Serialize)]
pub struct NavItem {
    /// Display title
    pub title: String,
    /// URL path
    pub path: String,
    /// Whether this item is currently active
    pub is_active: bool,
    /// Nested items (for collapsible sections)
    pub children: Vec<NavItem>,
    /// Whether this section is expanded
    pub is_expanded: bool,
    /// Chapter number (e.g., "1.2")
    pub number: Option<String>,
}

/// Context for rendering OOB sidebar.
#[derive(Debug, Clone, Serialize)]
pub struct SidebarContext {
    /// Navigation items
    pub items: Vec<NavItem>,
    /// Currently active path
    pub active_path: String,
}

/// Context for rendering OOB breadcrumb.
#[derive(Debug, Clone, Serialize)]
pub struct BreadcrumbContext {
    /// Breadcrumb trail
    pub crumbs: Vec<Breadcrumb>,
}

/// OOB update result containing all generated OOB HTML.
#[derive(Debug, Clone, Default)]
pub struct OobUpdates {
    /// Sidebar OOB HTML
    pub sidebar: Option<String>,
    /// Breadcrumb OOB HTML
    pub breadcrumb: Option<String>,
}

impl OobUpdates {
    /// Combine all OOB updates into a single HTML string.
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        if let Some(ref sidebar) = self.sidebar {
            html.push_str(sidebar);
        }

        if let Some(ref breadcrumb) = self.breadcrumb {
            html.push_str(breadcrumb);
        }

        html
    }

    /// Check if there are any OOB updates.
    pub fn is_empty(&self) -> bool {
        self.sidebar.is_none() && self.breadcrumb.is_none()
    }
}

/// Render OOB updates for a chapter.
///
/// # Arguments
/// * `tera` - Template engine
/// * `chapter` - Current chapter being rendered
/// * `all_chapters` - All chapters for sidebar navigation
/// * `active_path` - URL path of current page
///
/// # Returns
/// OOB updates to append to fragment response
pub fn render_oob_updates(
    tera: &Tera,
    chapter: &Chapter,
    all_chapters: &[&Chapter],
    active_path: &str,
) -> anyhow::Result<OobUpdates> {
    let mut updates = OobUpdates::default();

    // Build sidebar context
    let sidebar_items = build_nav_items(all_chapters, active_path);
    let sidebar_ctx = SidebarContext {
        items: sidebar_items,
        active_path: active_path.to_string(),
    };

    // Render sidebar OOB
    let mut ctx = Context::new();
    ctx.insert("sidebar", &sidebar_ctx);
    ctx.insert("active_path", &active_path);

    if let Ok(sidebar_html) = tera.render("partials/sidebar-oob.html", &ctx) {
        updates.sidebar = Some(format!(
            r#"<nav id="sidebar" hx-swap-oob="true">{}</nav>"#,
            sidebar_html
        ));
    }

    // Build breadcrumb context
    let crumbs = build_breadcrumbs(chapter, active_path);
    let breadcrumb_ctx = BreadcrumbContext { crumbs };

    // Render breadcrumb OOB
    ctx.insert("breadcrumb", &breadcrumb_ctx);

    if let Ok(breadcrumb_html) = tera.render("partials/breadcrumb.html", &ctx) {
        updates.breadcrumb = Some(format!(
            r#"<nav id="breadcrumb" aria-label="Breadcrumb" hx-swap-oob="true">{}</nav>"#,
            breadcrumb_html
        ));
    }

    Ok(updates)
}

/// Build navigation items from chapters.
fn build_nav_items(chapters: &[&Chapter], active_path: &str) -> Vec<NavItem> {
    chapters
        .iter()
        .filter_map(|ch| {
            ch.path.as_ref().map(|path| {
                let url_path = path_to_url(path);
                let is_active = url_path == active_path;
                let is_expanded = is_active || active_path.starts_with(&url_path);

                NavItem {
                    title: ch.name.clone(),
                    path: url_path,
                    is_active,
                    children: vec![], // TODO: Handle nested chapters
                    is_expanded,
                    number: ch.number.as_ref().map(|nums| {
                        nums.iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join(".")
                    }),
                }
            })
        })
        .collect()
}

/// Build breadcrumb trail for a chapter.
fn build_breadcrumbs(chapter: &Chapter, active_path: &str) -> Vec<Breadcrumb> {
    let mut crumbs = Vec::new();

    // Home crumb
    crumbs.push(Breadcrumb {
        title: "Home".to_string(),
        path: "/".to_string(),
        is_current: active_path == "/",
    });

    // Parent crumbs from parent_names
    for parent in chapter.parent_names.iter() {
        // Build path from parent structure
        // This is simplified - in practice, we'd need to track parent paths
        crumbs.push(Breadcrumb {
            title: parent.clone(),
            path: format!("/{}", parent.to_lowercase().replace(' ', "-")),
            is_current: false,
        });
    }

    // Current page crumb
    if active_path != "/" {
        crumbs.push(Breadcrumb {
            title: chapter.name.clone(),
            path: active_path.to_string(),
            is_current: true,
        });
    }

    crumbs
}

/// Convert file path to URL path.
fn path_to_url(path: &std::path::Path) -> String {
    let path_str = path.with_extension("").to_string_lossy().to_string();

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_breadcrumb_home() {
        let chapter = Chapter {
            name: "Home".to_string(),
            content: String::new(),
            path: Some(PathBuf::from("README.md")),
            source_path: None,
            number: None,
            sub_items: vec![],
            parent_names: vec![],
        };

        let crumbs = build_breadcrumbs(&chapter, "/");
        assert_eq!(crumbs.len(), 1);
        assert!(crumbs[0].is_current);
    }

    #[test]
    fn test_breadcrumb_nested() {
        let chapter = Chapter {
            name: "Installation".to_string(),
            content: String::new(),
            path: Some(PathBuf::from("guide/installation.md")),
            source_path: None,
            number: Some(vec![1, 2]),
            sub_items: vec![],
            parent_names: vec!["Guide".to_string()],
        };

        let crumbs = build_breadcrumbs(&chapter, "/guide/installation");
        assert_eq!(crumbs.len(), 3); // Home > Guide > Installation
        assert!(!crumbs[0].is_current); // Home
        assert!(!crumbs[1].is_current); // Guide
        assert!(crumbs[2].is_current); // Installation
    }

    #[test]
    fn test_nav_items() {
        let ch1 = Chapter {
            name: "Introduction".to_string(),
            content: String::new(),
            path: Some(PathBuf::from("intro.md")),
            source_path: None,
            number: Some(vec![1]),
            sub_items: vec![],
            parent_names: vec![],
        };

        let ch2 = Chapter {
            name: "Guide".to_string(),
            content: String::new(),
            path: Some(PathBuf::from("guide.md")),
            source_path: None,
            number: Some(vec![2]),
            sub_items: vec![],
            parent_names: vec![],
        };

        let chapters: Vec<&Chapter> = vec![&ch1, &ch2];
        let items = build_nav_items(&chapters, "/intro");

        assert_eq!(items.len(), 2);
        assert!(items[0].is_active);
        assert!(!items[1].is_active);
        assert_eq!(items[0].number, Some("1".to_string()));
    }

    #[test]
    fn test_oob_updates_to_html() {
        let updates = OobUpdates {
            sidebar: Some("<ul>sidebar</ul>".to_string()),
            breadcrumb: Some("<ol>crumbs</ol>".to_string()),
        };

        let html = updates.to_html();
        assert!(html.contains("sidebar"));
        assert!(html.contains("crumbs"));
    }

    #[test]
    fn test_oob_updates_empty() {
        let updates = OobUpdates::default();
        assert!(updates.is_empty());
        assert!(updates.to_html().is_empty());
    }
}
