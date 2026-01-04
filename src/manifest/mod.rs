//! Manifest generation for server-side integration.
//!
//! Generates manifest.json containing page metadata for server use.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The manifest file containing all page metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Schema version for compatibility checking
    #[serde(rename = "$schema")]
    pub schema: String,

    /// Manifest format version
    pub version: String,

    /// Build timestamp (ISO 8601)
    pub generated_at: String,

    /// Map of URL paths to page entries
    pub pages: HashMap<String, PageEntry>,
}

/// Metadata for a single page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageEntry {
    /// Page title
    pub title: String,

    /// Source file path relative to src/
    pub source: PathBuf,

    /// Path to full HTML page
    pub page_path: PathBuf,

    /// Path to content fragment
    pub fragment_path: PathBuf,

    /// Audience scope (if set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// Authentication requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authn: Option<String>,

    /// Required roles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authz: Option<Vec<String>>,

    /// Fallback page for access denied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,

    /// Content hash for cache invalidation
    pub content_hash: String,
}

impl Manifest {
    /// Create a new empty manifest.
    pub fn new() -> Self {
        Self {
            schema: "https://schemas.arusty.dev/mdbook-htmx/manifest.schema.json".to_string(),
            version: "1.0.0".to_string(),
            generated_at: chrono::Utc::now().to_rfc3339(),
            pages: HashMap::new(),
        }
    }

    /// Add a page entry to the manifest.
    pub fn add_page(&mut self, url_path: String, entry: PageEntry) {
        self.pages.insert(url_path, entry);
    }

    /// Serialize the manifest to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Self::new()
    }
}
