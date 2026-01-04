//! Configuration loading and validation for mdbook-htmx.
//!
//! Handles parsing of [output.htmx] configuration from book.toml.

use serde::{Deserialize, Serialize};

/// Configuration for the HTMX backend.
///
/// This is parsed from the `[output.htmx]` section of book.toml.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HtmxConfig {
    /// Output directory name (default: "htmx")
    pub output_dir: String,

    /// Enable HTMX-enhanced navigation (default: true)
    pub htmx_enabled: bool,

    /// Generate fragments alongside full pages (default: true)
    pub generate_fragments: bool,

    /// Generate manifest.json (default: true)
    pub generate_manifest: bool,

    /// Default audience scope for unscoped content
    pub default_scope: Option<String>,
}

impl Default for HtmxConfig {
    fn default() -> Self {
        Self {
            output_dir: "htmx".to_string(),
            htmx_enabled: true,
            generate_fragments: true,
            generate_manifest: true,
            default_scope: None,
        }
    }
}

impl HtmxConfig {
    /// Load configuration from the MDBook RenderContext config table.
    ///
    /// # Arguments
    /// * `config` - The config table from RenderContext
    ///
    /// # Returns
    /// Parsed HtmxConfig or error
    pub fn from_config(_config: &serde_json::Value) -> Result<Self, crate::BuildError> {
        // TODO: Implement in Stage 1
        Ok(Self::default())
    }
}
