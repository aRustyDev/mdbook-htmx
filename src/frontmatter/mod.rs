//! Frontmatter parsing and validation.
//!
//! Parses YAML frontmatter from chapter Markdown files.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::BuildError;

/// Parsed frontmatter from a chapter file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Frontmatter {
    /// Page title (overrides chapter title from SUMMARY.md)
    pub title: Option<String>,

    /// Page description for meta tags
    pub description: Option<String>,

    /// Audience scope for access control
    pub scope: Option<String>,

    /// Required authentication level
    pub authn: Option<AuthnLevel>,

    /// Required roles for access
    pub authz: Option<Vec<String>>,

    /// Fallback page path when access denied
    pub fallback: Option<String>,

    /// Custom template to use
    pub template: Option<String>,

    /// Exclude from search index
    #[serde(default)]
    pub no_search: bool,

    /// Exclude from navigation
    #[serde(default)]
    pub hidden: bool,
}

/// Authentication level requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthnLevel {
    /// No authentication required
    Public,
    /// Must be authenticated
    Authenticated,
    /// Must be authenticated and verified
    Verified,
}

/// Parse frontmatter from chapter content.
///
/// Frontmatter is delimited by `---` at the start of the file.
///
/// # Arguments
/// * `content` - The full chapter content including frontmatter
/// * `path` - Path to the source file (for error messages)
///
/// # Returns
/// Tuple of (frontmatter, remaining content)
pub fn parse_frontmatter<'a>(
    content: &'a str,
    path: &Path,
) -> Result<(Frontmatter, &'a str), BuildError> {
    // Check for frontmatter delimiter
    if !content.starts_with("---") {
        return Ok((Frontmatter::default(), content));
    }

    // Find closing delimiter
    let rest = &content[3..];
    if let Some(end) = rest.find("\n---") {
        let yaml = &rest[..end];
        let remaining = &rest[end + 4..].trim_start();

        let frontmatter: Frontmatter =
            serde_yaml::from_str(yaml).map_err(|e| BuildError::InvalidFrontmatter {
                path: path.to_path_buf(),
                source: e,
            })?;

        Ok((frontmatter, remaining))
    } else {
        // No closing delimiter, treat as regular content
        Ok((Frontmatter::default(), content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_no_frontmatter() {
        let content = "# Hello World\n\nSome content.";
        let (fm, remaining) = parse_frontmatter(content, &PathBuf::from("test.md")).unwrap();
        assert!(fm.title.is_none());
        assert_eq!(remaining, content);
    }

    #[test]
    fn test_with_frontmatter() {
        let content = "---\ntitle: My Title\nscope: internal\n---\n# Hello World";
        let (fm, remaining) = parse_frontmatter(content, &PathBuf::from("test.md")).unwrap();
        assert_eq!(fm.title, Some("My Title".to_string()));
        assert_eq!(fm.scope, Some("internal".to_string()));
        assert!(remaining.starts_with("# Hello World"));
    }
}
