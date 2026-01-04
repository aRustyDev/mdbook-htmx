//! mdbook-htmx library
//!
//! Provides the core rendering functionality for the mdbook-htmx backend.

pub mod assets;
pub mod config;
pub mod error;
pub mod frontmatter;
pub mod manifest;
pub mod render;
pub mod search;
pub mod templates;

use anyhow::Result;

pub use config::HtmxConfig;
pub use error::BuildError;

/// The main renderer that processes MDBook content and produces HTMX-enhanced HTML.
pub struct HtmxRenderer {
    // TODO: Add fields in Stage 1
    _private: (),
}

impl HtmxRenderer {
    /// Create a new renderer from JSON RenderContext.
    ///
    /// # Arguments
    /// * `json` - The JSON string containing the MDBook RenderContext
    ///
    /// # Returns
    /// A configured HtmxRenderer ready to render
    pub fn from_json(_json: &str) -> Result<Self> {
        // TODO: Implement in Stage 1
        Ok(Self { _private: () })
    }

    /// Render the book to the output directory.
    ///
    /// This produces:
    /// - `book/htmx/pages/*.html` - Full HTML pages
    /// - `book/htmx/fragments/*.html` - Content-only fragments for HTMX
    /// - `book/htmx/manifest.json` - Page metadata for server integration
    pub fn render(&self) -> Result<()> {
        // TODO: Implement in Stage 1
        Ok(())
    }
}
