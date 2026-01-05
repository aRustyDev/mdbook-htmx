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

impl AuthnLevel {
    /// Get the string representation matching serde serialization.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Authenticated => "authenticated",
            Self::Verified => "verified",
        }
    }
}

impl std::fmt::Display for AuthnLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
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

    #[test]
    fn test_authn_level_display() {
        assert_eq!(AuthnLevel::Public.to_string(), "public");
        assert_eq!(AuthnLevel::Authenticated.to_string(), "authenticated");
        assert_eq!(AuthnLevel::Verified.to_string(), "verified");
    }

    #[test]
    fn test_authn_level_as_str() {
        assert_eq!(AuthnLevel::Public.as_str(), "public");
        assert_eq!(AuthnLevel::Authenticated.as_str(), "authenticated");
        assert_eq!(AuthnLevel::Verified.as_str(), "verified");
    }

    #[test]
    fn test_frontmatter_with_authn() {
        let content = "---\ntitle: Admin Page\nauthn: authenticated\n---\n# Admin";
        let (fm, _) = parse_frontmatter(content, &PathBuf::from("admin.md")).unwrap();
        assert_eq!(fm.title, Some("Admin Page".to_string()));
        assert!(matches!(fm.authn, Some(AuthnLevel::Authenticated)));
    }

    #[test]
    fn test_frontmatter_with_authz() {
        let content = "---\ntitle: Admin Page\nauthz:\n  - admin\n  - editor\n---\n# Admin";
        let (fm, _) = parse_frontmatter(content, &PathBuf::from("admin.md")).unwrap();
        assert_eq!(
            fm.authz,
            Some(vec!["admin".to_string(), "editor".to_string()])
        );
    }

    #[test]
    fn test_frontmatter_with_fallback() {
        let content = "---\ntitle: Secret\nfallback: /docs/access-denied\n---\n# Secret";
        let (fm, _) = parse_frontmatter(content, &PathBuf::from("secret.md")).unwrap();
        assert_eq!(fm.fallback, Some("/docs/access-denied".to_string()));
    }

    #[test]
    fn test_frontmatter_full_auth() {
        let content = r#"---
title: Protected Content
authn: verified
authz:
  - admin
fallback: /signin
---
# Protected"#;
        let (fm, remaining) = parse_frontmatter(content, &PathBuf::from("protected.md")).unwrap();
        assert_eq!(fm.title, Some("Protected Content".to_string()));
        assert!(matches!(fm.authn, Some(AuthnLevel::Verified)));
        assert_eq!(fm.authz, Some(vec!["admin".to_string()]));
        assert_eq!(fm.fallback, Some("/signin".to_string()));
        assert!(remaining.starts_with("# Protected"));
    }
}
