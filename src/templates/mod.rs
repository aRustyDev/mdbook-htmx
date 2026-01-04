//! Template engine setup and custom functions.
//!
//! Configures Tera with embedded templates and custom filters/functions.

use anyhow::Result;
use tera::Tera;

/// Embedded templates compiled into the binary.
const BUILTIN_TEMPLATES: &[(&str, &str)] = &[
    ("layout.html", include_str!("../../templates/layout.html")),
    (
        "docs/page.html",
        include_str!("../../templates/docs/page.html"),
    ),
    (
        "docs/fragment.html",
        include_str!("../../templates/docs/fragment.html"),
    ),
];

/// Initialize the Tera template engine with embedded templates.
///
/// Templates are embedded at compile time from the templates/ directory.
///
/// # Returns
/// Configured Tera instance
pub fn init_templates() -> Result<Tera> {
    let mut tera = Tera::default();

    // Add embedded templates
    tera.add_raw_templates(BUILTIN_TEMPLATES.iter().copied())?;

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

    // truncate_words: Truncate text to N words
    tera.register_filter("truncate_words", truncate_words_filter);
}

/// Register custom Tera functions.
fn register_functions(_tera: &mut Tera) {
    // TODO: Register functions like asset_url, fragment_url in Stage 2
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

/// Truncate text to a specified number of words.
fn truncate_words_filter(
    value: &tera::Value,
    args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("truncate_words requires a string"))?;

    let count = args.get("count").and_then(|v| v.as_u64()).unwrap_or(50) as usize;

    let words: Vec<&str> = s.split_whitespace().collect();

    if words.len() <= count {
        Ok(tera::Value::String(s.to_string()))
    } else {
        let truncated = words[..count].join(" ");
        Ok(tera::Value::String(format!("{}...", truncated)))
    }
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

    #[test]
    fn test_truncate_words() {
        let value = tera::Value::String("one two three four five".to_string());
        let mut args = HashMap::new();
        args.insert("count".to_string(), tera::Value::Number(3.into()));

        let result = truncate_words_filter(&value, &args).unwrap();
        assert_eq!(result, tera::Value::String("one two three...".to_string()));
    }

    #[test]
    fn test_init_templates() {
        let tera = init_templates().expect("Failed to initialize templates");
        assert!(tera.get_template_names().any(|n| n == "layout.html"));
        assert!(tera.get_template_names().any(|n| n == "docs/page.html"));
        assert!(tera.get_template_names().any(|n| n == "docs/fragment.html"));
    }
}
