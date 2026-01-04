//! Configuration loading and validation for mdbook-htmx.
//!
//! Handles parsing of [output.htmx] configuration from book.toml.

use serde::{Deserialize, Serialize};

use crate::BuildError;

/// Configuration for the HTMX backend.
///
/// This is parsed from the `[output.htmx]` section of book.toml.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct HtmxConfig {
    /// Schema version for config validation
    #[serde(default = "defaults::version")]
    pub version: String,

    /// HTMX library version to use
    #[serde(default = "defaults::htmx_version")]
    pub htmx_version: String,

    /// Enable hx-boost on body (default: true)
    #[serde(default = "defaults::boost")]
    pub boost: bool,

    /// Default swap strategy (default: innerHTML)
    #[serde(default = "defaults::swap_strategy")]
    pub swap_strategy: SwapStrategy,

    /// Default HTMX target selector (default: #content)
    #[serde(default = "defaults::target")]
    pub target: String,

    /// Push URL on navigation (default: true)
    #[serde(default = "defaults::push_url")]
    pub push_url: bool,

    /// Output mode: full pages, fragments, or both
    #[serde(default)]
    pub output_mode: OutputMode,

    /// Navigation settings
    #[serde(default)]
    pub navigation: NavigationConfig,

    /// Search settings
    #[serde(default)]
    pub search: SearchConfig,

    /// Asset handling settings
    #[serde(default)]
    pub assets: AssetsConfig,

    /// Default scope for unscoped content
    pub default_scope: Option<String>,

    /// Custom theme directory path
    pub theme_dir: Option<String>,
}

/// HTMX swap strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SwapStrategy {
    #[default]
    InnerHTML,
    OuterHTML,
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd,
    Delete,
    None,
}

impl std::fmt::Display for SwapStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InnerHTML => write!(f, "innerHTML"),
            Self::OuterHTML => write!(f, "outerHTML"),
            Self::BeforeBegin => write!(f, "beforebegin"),
            Self::AfterBegin => write!(f, "afterbegin"),
            Self::BeforeEnd => write!(f, "beforeend"),
            Self::AfterEnd => write!(f, "afterend"),
            Self::Delete => write!(f, "delete"),
            Self::None => write!(f, "none"),
        }
    }
}

/// Output mode configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    /// Generate only full pages
    Full,
    /// Generate only fragments
    Fragments,
    /// Generate both full pages and fragments (default)
    #[default]
    Both,
}

/// Navigation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct NavigationConfig {
    /// Show breadcrumb navigation
    pub breadcrumbs: bool,
    /// Show table of contents
    pub toc: bool,
    /// Show previous/next navigation
    pub prev_next: bool,
    /// Collapsible sidebar sections
    pub collapsible_sidebar: bool,
}

impl Default for NavigationConfig {
    fn default() -> Self {
        Self {
            breadcrumbs: true,
            toc: true,
            prev_next: true,
            collapsible_sidebar: true,
        }
    }
}

/// Search configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct SearchConfig {
    /// Enable search functionality
    pub enabled: bool,
    /// Generate search index
    pub generate_index: bool,
    /// Include content in search index (larger file size)
    pub index_content: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            generate_index: true,
            index_content: true,
        }
    }
}

/// Asset handling configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct AssetsConfig {
    /// Hash assets for cache busting
    pub hash_assets: bool,
    /// Generate SRI integrity hashes
    pub sri_enabled: bool,
    /// Copy additional assets from this directory
    pub additional_assets: Option<String>,
}

impl Default for AssetsConfig {
    fn default() -> Self {
        Self {
            hash_assets: true,
            sri_enabled: true,
            additional_assets: None,
        }
    }
}

impl Default for HtmxConfig {
    fn default() -> Self {
        Self {
            version: defaults::version(),
            htmx_version: defaults::htmx_version(),
            boost: defaults::boost(),
            swap_strategy: defaults::swap_strategy(),
            target: defaults::target(),
            push_url: defaults::push_url(),
            output_mode: OutputMode::default(),
            navigation: NavigationConfig::default(),
            search: SearchConfig::default(),
            assets: AssetsConfig::default(),
            default_scope: None,
            theme_dir: None,
        }
    }
}

mod defaults {
    use super::SwapStrategy;

    pub fn version() -> String {
        "1.0".to_string()
    }

    pub fn htmx_version() -> String {
        "1.9.10".to_string()
    }

    pub fn boost() -> bool {
        true
    }

    pub fn swap_strategy() -> SwapStrategy {
        SwapStrategy::InnerHTML
    }

    pub fn target() -> String {
        "#content".to_string()
    }

    pub fn push_url() -> bool {
        true
    }
}

impl HtmxConfig {
    /// Load configuration from the MDBook RenderContext output config.
    ///
    /// # Arguments
    /// * `output_config` - The output.htmx table from RenderContext
    ///
    /// # Returns
    /// Parsed HtmxConfig or error
    pub fn from_toml(output_config: Option<&toml::Value>) -> Result<Self, BuildError> {
        match output_config {
            Some(value) => {
                let config: HtmxConfig = value.clone().try_into().map_err(|e| {
                    BuildError::ConfigError(format!("Failed to parse [output.htmx]: {}", e))
                })?;
                Ok(config)
            }
            None => Ok(Self::default()),
        }
    }

    /// Validate the configuration.
    pub fn validate(&self) -> Result<(), BuildError> {
        // Validate version format
        if !self.version.starts_with("1.") {
            return Err(BuildError::ConfigError(format!(
                "Unsupported config version: {}. Expected 1.x",
                self.version
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = HtmxConfig::default();
        assert!(config.boost);
        assert_eq!(config.target, "#content");
        assert_eq!(config.output_mode, OutputMode::Both);
    }

    #[test]
    fn test_swap_strategy_display() {
        assert_eq!(SwapStrategy::InnerHTML.to_string(), "innerHTML");
        assert_eq!(SwapStrategy::OuterHTML.to_string(), "outerHTML");
    }
}
