//! HTMX attribute injection.
//!
//! Injects HTMX attributes into HTML for SPA-like navigation.

use crate::config::{HtmxConfig, SwapStrategy};

/// Inject HTMX attributes into rendered HTML.
///
/// Adds:
/// - `hx-boost="true"` on `<body>` (if boost enabled)
/// - `hx-target` and `hx-swap` defaults
/// - `hx-push-url="true"` on navigation links (if push_url enabled)
pub fn inject_htmx_attrs(html: &str, config: &HtmxConfig) -> String {
    let mut html = html.to_string();

    // Add hx-boost to body
    if config.boost {
        html = inject_body_attrs(&html, config);
    }

    // Add hx-push-url to navigation links
    if config.push_url {
        html = inject_push_url(&html);
    }

    html
}

/// Convert SwapStrategy to its HTMX string representation.
fn swap_strategy_to_str(strategy: &SwapStrategy) -> &'static str {
    match strategy {
        SwapStrategy::InnerHTML => "innerHTML",
        SwapStrategy::OuterHTML => "outerHTML",
        SwapStrategy::BeforeBegin => "beforebegin",
        SwapStrategy::AfterBegin => "afterbegin",
        SwapStrategy::BeforeEnd => "beforeend",
        SwapStrategy::AfterEnd => "afterend",
        SwapStrategy::Delete => "delete",
        SwapStrategy::None => "none",
    }
}

/// Inject HTMX attributes on the `<body>` tag.
fn inject_body_attrs(html: &str, config: &HtmxConfig) -> String {
    let swap_str = swap_strategy_to_str(&config.swap_strategy);

    // Find <body and inject attributes
    if let Some(body_pos) = html.find("<body") {
        let (before, after) = html.split_at(body_pos);
        // Find the end of the opening tag
        if let Some(tag_end) = after.find('>') {
            let (tag_start, rest) = after.split_at(tag_end);
            // Check if there are existing attributes
            let attrs = format!(
                " hx-boost=\"true\" hx-target=\"{}\" hx-swap=\"{}\"",
                config.target, swap_str
            );

            // Insert before the closing >
            return format!("{}{}{}{}", before, tag_start, attrs, rest);
        }
    }

    html.to_string()
}

/// Inject `hx-push-url="true"` on navigation links.
///
/// Targets links with class `nav-link` or within navigation elements.
fn inject_push_url(html: &str) -> String {
    // For now, we rely on template-level attributes rather than post-processing.
    // The templates include hx-push-url on navigation links.
    // This function is here for future enhancements like link processing.
    html.to_string()
}

/// Generate HTMX attributes for a navigation link.
///
/// Returns a string of HTMX attributes to add to an `<a>` tag.
pub fn nav_link_attrs(target: &str, swap: &SwapStrategy, push_url: bool) -> String {
    let swap_str = swap_strategy_to_str(swap);
    let swap_with_scroll = format!("{} show:window:top", swap_str);

    let mut attrs = format!(
        "hx-boost=\"true\" hx-target=\"{}\" hx-swap=\"{}\"",
        target, swap_with_scroll
    );

    if push_url {
        attrs.push_str(" hx-push-url=\"true\"");
    }

    attrs
}

/// Generate preload hint for hover prefetching.
pub fn preload_hint(path: &str) -> String {
    format!(r#"<link rel="prefetch" href="{}" as="document">"#, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_body_attrs() {
        let html = r#"<html><body class="main"><div>Content</div></body></html>"#;
        let config = HtmxConfig::default();

        let result = inject_htmx_attrs(html, &config);

        assert!(result.contains("hx-boost=\"true\""));
        assert!(result.contains("hx-target=\"#content\""));
        assert!(result.contains("hx-swap=\"innerHTML\""));
    }

    #[test]
    fn test_inject_body_attrs_custom_target() {
        let html = r#"<html><body><div>Content</div></body></html>"#;
        let mut config = HtmxConfig::default();
        config.target = "#main".to_string();
        config.swap_strategy = SwapStrategy::OuterHTML;

        let result = inject_htmx_attrs(html, &config);

        assert!(result.contains("hx-target=\"#main\""));
        assert!(result.contains("hx-swap=\"outerHTML\""));
    }

    #[test]
    fn test_no_boost() {
        let html = r#"<html><body><div>Content</div></body></html>"#;
        let mut config = HtmxConfig::default();
        config.boost = false;

        let result = inject_htmx_attrs(html, &config);

        assert!(!result.contains("hx-boost"));
    }

    #[test]
    fn test_nav_link_attrs() {
        let attrs = nav_link_attrs("#content", &SwapStrategy::InnerHTML, true);

        assert!(attrs.contains("hx-boost=\"true\""));
        assert!(attrs.contains("hx-target=\"#content\""));
        assert!(attrs.contains("hx-push-url=\"true\""));
    }

    #[test]
    fn test_preload_hint() {
        let hint = preload_hint("/chapter/intro");
        assert_eq!(
            hint,
            r#"<link rel="prefetch" href="/chapter/intro" as="document">"#
        );
    }

    #[test]
    fn test_swap_strategy_strings() {
        assert_eq!(swap_strategy_to_str(&SwapStrategy::InnerHTML), "innerHTML");
        assert_eq!(swap_strategy_to_str(&SwapStrategy::OuterHTML), "outerHTML");
        assert_eq!(
            swap_strategy_to_str(&SwapStrategy::BeforeBegin),
            "beforebegin"
        );
        assert_eq!(swap_strategy_to_str(&SwapStrategy::AfterEnd), "afterend");
    }
}
