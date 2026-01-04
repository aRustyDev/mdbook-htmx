//! Template engine setup and custom functions.
//!
//! Configures Tera with embedded templates and custom filters/functions.

use anyhow::Result;
use tera::Tera;

/// Initialize the Tera template engine with embedded templates.
///
/// Templates are embedded at compile time from the templates/ directory.
///
/// # Returns
/// Configured Tera instance
pub fn init_templates() -> Result<Tera> {
    let mut tera = Tera::default();

    // TODO: In Stage 1, embed templates using include_str!
    // For now, return empty Tera instance

    // Register custom filters
    register_filters(&mut tera);

    // Register custom functions
    register_functions(&mut tera);

    Ok(tera)
}

/// Register custom Tera filters.
fn register_filters(tera: &mut Tera) {
    // slugify: Convert string to URL-safe slug
    tera.register_filter("slugify", slugify_filter);

    // TODO: Add more filters in Stage 1
}

/// Register custom Tera functions.
fn register_functions(tera: &mut Tera) {
    // TODO: Register functions like asset_url, fragment_url in Stage 2
    let _ = tera; // Suppress unused warning for now
}

/// Slugify filter: converts "Hello World" to "hello-world".
fn slugify_filter(
    value: &tera::Value,
    _args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("slugify requires a string"))?;

    let slug = s
        .to_lowercase()
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
        .join("-");

    Ok(tera::Value::String(slug))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_slugify() {
        let value = tera::Value::String("Hello World".to_string());
        let result = slugify_filter(&value, &HashMap::new()).unwrap();
        assert_eq!(result, tera::Value::String("hello-world".to_string()));
    }

    #[test]
    fn test_slugify_special_chars() {
        let value = tera::Value::String("API & REST: A Guide".to_string());
        let result = slugify_filter(&value, &HashMap::new()).unwrap();
        assert_eq!(result, tera::Value::String("api-rest-a-guide".to_string()));
    }
}
