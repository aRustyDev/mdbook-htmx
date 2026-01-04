//! mdbook-htmx - HTMX-enhanced alternative backend for mdBook
//!
//! This backend produces HTMX-enhanced HTML documentation with:
//! - Server-side rendering capabilities
//! - Authorization metadata for access control
//! - Audience-scoped content filtering
//! - SPA-like navigation without JavaScript frameworks

use std::io::{self, Read};

use anyhow::{Context, Result};
use log::{debug, error, info};

use mdbook_htmx::HtmxRenderer;

fn main() -> Result<()> {
    env_logger::init();

    info!("mdbook-htmx v{}", env!("CARGO_PKG_VERSION"));

    // MDBook passes RenderContext via stdin as JSON
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin
        .read_to_string(&mut input)
        .context("Failed to read RenderContext from stdin")?;

    debug!("Received {} bytes from stdin", input.len());

    // Parse and render
    let renderer = HtmxRenderer::from_json(&input)?;

    match renderer.render() {
        Ok(()) => {
            info!("Rendering complete");
            Ok(())
        }
        Err(e) => {
            error!("Rendering failed: {}", e);
            Err(e)
        }
    }
}
