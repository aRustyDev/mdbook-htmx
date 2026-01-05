//! Search index generation.
//!
//! Generates search-index.json for client-side or server-side search.
//! See ADR-0005 and ADR-0021 for design decisions.

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};

use crate::config::SearchConfig;
use crate::frontmatter::Frontmatter;

/// The search index containing all searchable content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndex {
    /// Schema version URL
    #[serde(rename = "$schema")]
    pub schema: String,

    /// Index format version
    pub version: String,

    /// Index configuration (for client-side search reference)
    pub config: SearchIndexConfig,

    /// Indexed documents
    pub documents: Vec<SearchDocument>,
}

/// Search index configuration embedded in the index.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchIndexConfig {
    /// Maximum heading level indexed
    pub heading_split_level: u8,
    /// Whether body content is included
    pub include_body: bool,
    /// Whether auth info is included
    pub include_auth: bool,
}

/// A searchable document entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocument {
    /// Document path (URL path)
    pub path: String,

    /// Page title
    pub title: String,

    /// Plain text content for search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Headings with anchors for deep linking
    pub headings: Vec<HeadingEntry>,

    /// Authentication/authorization metadata for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<DocumentAuth>,

    /// Audience scope (for filtering)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// A heading entry for anchor linking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingEntry {
    /// Heading level (1-6)
    pub level: u8,

    /// Heading text
    pub text: String,

    /// Anchor ID (e.g., "#installation")
    pub anchor: String,
}

/// Authentication/authorization metadata for a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentAuth {
    /// Required authentication level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authn: Option<String>,

    /// Required roles for access
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authz: Option<Vec<String>>,
}

impl SearchIndex {
    /// Create a new empty search index with the given configuration.
    pub fn new(config: &SearchConfig) -> Self {
        Self {
            schema: "https://schemas.arusty.dev/mdbook-htmx/search-index.schema.json".to_string(),
            version: "1.0.0".to_string(),
            config: SearchIndexConfig {
                heading_split_level: config.heading_split_level,
                include_body: config.index_content,
                include_auth: config.include_auth,
            },
            documents: Vec::new(),
        }
    }

    /// Add a document to the index.
    pub fn add_document(&mut self, doc: SearchDocument) {
        self.documents.push(doc);
    }

    /// Serialize the index to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Filter index by scope.
    ///
    /// Returns a new index containing only documents matching the given scope.
    pub fn filter_by_scope(&self, scope: &str) -> Self {
        let documents: Vec<_> = self
            .documents
            .iter()
            .filter(|doc| doc.scope.as_deref() == Some(scope) || doc.scope.is_none())
            .cloned()
            .collect();

        Self {
            schema: self.schema.clone(),
            version: self.version.clone(),
            config: self.config.clone(),
            documents,
        }
    }

    /// Filter index by authentication level.
    ///
    /// Returns a new index containing only documents accessible at the given level.
    pub fn filter_by_auth(&self, level: &str) -> Self {
        let documents: Vec<_> = self
            .documents
            .iter()
            .filter(|doc| {
                match &doc.auth {
                    None => true, // No auth requirement
                    Some(auth) => {
                        match &auth.authn {
                            None => true,
                            Some(required) => {
                                // "public" is accessible to all
                                // "authenticated" requires authentication
                                // "verified" requires verified authentication
                                required == "public"
                                    || (level == "authenticated" && required != "verified")
                                    || level == "verified"
                            }
                        }
                    }
                }
            })
            .cloned()
            .collect();

        Self {
            schema: self.schema.clone(),
            version: self.version.clone(),
            config: self.config.clone(),
            documents,
        }
    }
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new(&SearchConfig::default())
    }
}

/// Builder for creating search documents from chapter content.
pub struct SearchDocumentBuilder<'a> {
    config: &'a SearchConfig,
}

impl<'a> SearchDocumentBuilder<'a> {
    /// Create a new document builder with the given configuration.
    pub fn new(config: &'a SearchConfig) -> Self {
        Self { config }
    }

    /// Build a search document from chapter content.
    ///
    /// # Arguments
    /// * `path` - URL path for the document
    /// * `title` - Page title
    /// * `content` - Markdown content (after frontmatter extraction)
    /// * `frontmatter` - Parsed frontmatter
    ///
    /// # Returns
    /// A `SearchDocument` if the page should be indexed, None if `no_search` is set.
    pub fn build(
        &self,
        path: String,
        title: String,
        content: &str,
        frontmatter: &Frontmatter,
    ) -> Option<SearchDocument> {
        // Skip if no_search is set
        if frontmatter.no_search {
            return None;
        }

        // Extract headings up to configured level
        let headings = self.extract_headings(content);

        // Extract body text if configured
        let body = if self.config.index_content {
            let text = strip_markdown(content);
            let text = if let Some(max_len) = self.config.max_excerpt_length {
                truncate_text(&text, max_len)
            } else {
                text
            };
            Some(text)
        } else {
            None
        };

        // Build auth metadata if configured
        let auth = if self.config.include_auth {
            if frontmatter.authn.is_some() || frontmatter.authz.is_some() {
                Some(DocumentAuth {
                    authn: frontmatter.authn.as_ref().map(|a| a.to_string()),
                    authz: frontmatter.authz.clone(),
                })
            } else {
                None
            }
        } else {
            None
        };

        Some(SearchDocument {
            path,
            title,
            body,
            headings,
            auth,
            scope: frontmatter.scope.clone(),
        })
    }

    /// Extract headings from markdown up to the configured level.
    fn extract_headings(&self, markdown: &str) -> Vec<HeadingEntry> {
        use pulldown_cmark::HeadingLevel;

        let options = Options::ENABLE_HEADING_ATTRIBUTES;
        let parser = Parser::new_ext(markdown, options);

        let mut headings = Vec::new();
        let mut in_heading = false;
        let mut current_level = 0u8;
        let mut current_text = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    current_level = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    current_text.clear();
                }
                Event::End(TagEnd::Heading(_)) => {
                    if in_heading && current_level <= self.config.heading_split_level {
                        let anchor = format!("#{}", slugify(&current_text));
                        headings.push(HeadingEntry {
                            level: current_level,
                            text: current_text.clone(),
                            anchor,
                        });
                    }
                    in_heading = false;
                }
                Event::Text(text) | Event::Code(text) => {
                    if in_heading {
                        current_text.push_str(&text);
                    }
                }
                _ => {}
            }
        }

        headings
    }
}

/// Strip markdown syntax to get plain text.
///
/// Removes:
/// - Code blocks
/// - Inline code
/// - Links (keeps text)
/// - Images
/// - HTML tags
/// - Formatting markers
pub fn strip_markdown(markdown: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;

    let parser = Parser::new_ext(markdown, options);

    let mut output = String::new();
    let mut skip_depth = 0;

    for event in parser {
        match event {
            // Skip code blocks entirely
            Event::Start(Tag::CodeBlock(_)) => {
                skip_depth += 1;
            }
            Event::End(TagEnd::CodeBlock) => {
                skip_depth -= 1;
            }
            // Skip inline code
            Event::Code(_) => {}
            // Skip images
            Event::Start(Tag::Image { .. }) => {
                skip_depth += 1;
            }
            Event::End(TagEnd::Image) => {
                skip_depth -= 1;
            }
            // Keep text content
            Event::Text(text) => {
                if skip_depth == 0 {
                    output.push_str(&text);
                    output.push(' ');
                }
            }
            // Add newlines for structure
            Event::SoftBreak | Event::HardBreak => {
                if skip_depth == 0 {
                    output.push(' ');
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if skip_depth == 0 {
                    output.push('\n');
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if skip_depth == 0 {
                    output.push('\n');
                }
            }
            _ => {}
        }
    }

    // Clean up whitespace
    output.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Convert text to a URL-safe slug.
///
/// Uses GitHub-compatible slugification.
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '_' {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Truncate text to a maximum length, breaking at word boundaries.
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        return text.to_string();
    }

    // Find last space before max_len
    let truncated = &text[..max_len];
    if let Some(last_space) = truncated.rfind(' ') {
        format!("{}...", &text[..last_space])
    } else {
        format!("{}...", truncated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontmatter::AuthnLevel;

    #[test]
    fn test_search_index_new() {
        let config = SearchConfig::default();
        let index = SearchIndex::new(&config);

        assert_eq!(index.version, "1.0.0");
        assert_eq!(index.config.heading_split_level, 3);
        assert!(index.config.include_body);
        assert!(index.config.include_auth);
        assert!(index.documents.is_empty());
    }

    #[test]
    fn test_strip_markdown_basic() {
        let md = "# Hello World\n\nThis is **bold** and *italic* text.";
        let plain = strip_markdown(md);
        assert!(plain.contains("Hello World"));
        assert!(plain.contains("bold"));
        assert!(plain.contains("italic"));
        assert!(!plain.contains("**"));
        assert!(!plain.contains("*"));
    }

    #[test]
    fn test_strip_markdown_code() {
        let md = "Some text\n\n```rust\nfn main() {}\n```\n\nMore text";
        let plain = strip_markdown(md);
        assert!(plain.contains("Some text"));
        assert!(plain.contains("More text"));
        assert!(!plain.contains("fn main"));
    }

    #[test]
    fn test_strip_markdown_links() {
        let md = "Check out [this link](https://example.com) for more.";
        let plain = strip_markdown(md);
        assert!(plain.contains("this link"));
        assert!(!plain.contains("https://"));
    }

    #[test]
    fn test_strip_markdown_inline_code() {
        let md = "Use the `foo()` function.";
        let plain = strip_markdown(md);
        assert!(plain.contains("Use the"));
        assert!(plain.contains("function"));
        // Inline code is stripped
        assert!(!plain.contains("`"));
    }

    #[test]
    fn test_extract_headings() {
        let config = SearchConfig {
            heading_split_level: 3,
            ..Default::default()
        };
        let builder = SearchDocumentBuilder::new(&config);

        let md = "# Title\n\n## Section 1\n\n### Subsection\n\n#### Too Deep";
        let headings = builder.extract_headings(md);

        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0].text, "Title");
        assert_eq!(headings[0].level, 1);
        assert_eq!(headings[0].anchor, "#title");
        assert_eq!(headings[1].text, "Section 1");
        assert_eq!(headings[2].text, "Subsection");
        // H4 should be excluded with heading_split_level=3
    }

    #[test]
    fn test_extract_headings_level_limit() {
        let config = SearchConfig {
            heading_split_level: 2,
            ..Default::default()
        };
        let builder = SearchDocumentBuilder::new(&config);

        let md = "# Title\n\n## Section\n\n### Subsection";
        let headings = builder.extract_headings(md);

        assert_eq!(headings.len(), 2);
        assert_eq!(headings[0].text, "Title");
        assert_eq!(headings[1].text, "Section");
    }

    #[test]
    fn test_truncate_text() {
        let text = "This is a long piece of text that needs truncation.";
        let truncated = truncate_text(text, 20);
        assert!(truncated.ends_with("..."));
        assert!(truncated.len() < text.len());
    }

    #[test]
    fn test_truncate_text_short() {
        let text = "Short";
        let truncated = truncate_text(text, 20);
        assert_eq!(truncated, "Short");
    }

    #[test]
    fn test_document_builder_no_search() {
        let config = SearchConfig::default();
        let builder = SearchDocumentBuilder::new(&config);

        let frontmatter = Frontmatter {
            no_search: true,
            ..Default::default()
        };

        let result = builder.build(
            "/test".to_string(),
            "Test".to_string(),
            "# Content",
            &frontmatter,
        );

        assert!(result.is_none());
    }

    #[test]
    fn test_document_builder_with_auth() {
        let config = SearchConfig {
            include_auth: true,
            ..Default::default()
        };
        let builder = SearchDocumentBuilder::new(&config);

        let frontmatter = Frontmatter {
            authn: Some(AuthnLevel::Authenticated),
            authz: Some(vec!["admin".to_string()]),
            ..Default::default()
        };

        let doc = builder
            .build(
                "/admin".to_string(),
                "Admin Page".to_string(),
                "# Admin",
                &frontmatter,
            )
            .unwrap();

        assert!(doc.auth.is_some());
        let auth = doc.auth.unwrap();
        assert_eq!(auth.authn, Some("authenticated".to_string()));
        assert_eq!(auth.authz, Some(vec!["admin".to_string()]));
    }

    #[test]
    fn test_document_builder_with_body() {
        let config = SearchConfig {
            index_content: true,
            max_excerpt_length: Some(50),
            ..Default::default()
        };
        let builder = SearchDocumentBuilder::new(&config);

        let frontmatter = Frontmatter::default();

        let doc = builder
            .build(
                "/test".to_string(),
                "Test".to_string(),
                "# Title\n\nThis is some long content that will be truncated because it exceeds the maximum length.",
                &frontmatter,
            )
            .unwrap();

        assert!(doc.body.is_some());
        let body = doc.body.unwrap();
        assert!(body.ends_with("..."));
    }

    #[test]
    fn test_filter_by_scope() {
        let config = SearchConfig::default();
        let mut index = SearchIndex::new(&config);

        index.add_document(SearchDocument {
            path: "/public".to_string(),
            title: "Public".to_string(),
            body: None,
            headings: vec![],
            auth: None,
            scope: None,
        });

        index.add_document(SearchDocument {
            path: "/internal".to_string(),
            title: "Internal".to_string(),
            body: None,
            headings: vec![],
            auth: None,
            scope: Some("internal".to_string()),
        });

        let filtered = index.filter_by_scope("internal");
        assert_eq!(filtered.documents.len(), 2); // public (no scope) + internal
    }

    #[test]
    fn test_filter_by_auth() {
        let config = SearchConfig::default();
        let mut index = SearchIndex::new(&config);

        index.add_document(SearchDocument {
            path: "/public".to_string(),
            title: "Public".to_string(),
            body: None,
            headings: vec![],
            auth: Some(DocumentAuth {
                authn: Some("public".to_string()),
                authz: None,
            }),
            scope: None,
        });

        index.add_document(SearchDocument {
            path: "/private".to_string(),
            title: "Private".to_string(),
            body: None,
            headings: vec![],
            auth: Some(DocumentAuth {
                authn: Some("authenticated".to_string()),
                authz: None,
            }),
            scope: None,
        });

        let public_filtered = index.filter_by_auth("public");
        assert_eq!(public_filtered.documents.len(), 1);
        assert_eq!(public_filtered.documents[0].path, "/public");

        let auth_filtered = index.filter_by_auth("authenticated");
        assert_eq!(auth_filtered.documents.len(), 2);
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("API & REST"), "api-rest");
        assert_eq!(slugify("  Multiple   Spaces  "), "multiple-spaces");
    }
}
