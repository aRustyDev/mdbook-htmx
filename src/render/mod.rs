//! Rendering logic for mdbook-htmx.
//!
//! Handles Markdown to HTML conversion using pulldown-cmark.

use pulldown_cmark::{html, Options, Parser};

/// Convert Markdown content to HTML.
///
/// Uses pulldown-cmark with GitHub-flavored Markdown extensions.
///
/// # Arguments
/// * `markdown` - The Markdown source
///
/// # Returns
/// HTML string
pub fn markdown_to_html(markdown: &str) -> String {
    // Enable common extensions
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES;

    let parser = Parser::new_ext(markdown, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

/// Extract headings from Markdown for table of contents.
///
/// # Arguments
/// * `markdown` - The Markdown source
///
/// # Returns
/// Vector of (level, text, anchor) tuples
pub fn extract_headings(markdown: &str) -> Vec<(u8, String, String)> {
    use pulldown_cmark::{Event, HeadingLevel, Tag, TagEnd};

    let options = Options::ENABLE_HEADING_ATTRIBUTES;
    let parser = Parser::new_ext(markdown, options);

    let mut headings = Vec::new();
    let mut in_heading = false;
    let mut current_level = 0u8;
    let mut current_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                current_level = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                current_text.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                if in_heading {
                    let anchor = slugify(&current_text);
                    headings.push((current_level, current_text.clone(), anchor));
                    in_heading = false;
                }
            }
            Event::Text(text) | Event::Code(text) => {
                if in_heading {
                    current_text.push_str(&text);
                }
            }
            _ => {}
        }
    }

    headings
}

/// Convert text to a URL-safe slug.
///
/// Uses GitHub-compatible slugification:
/// 1. Convert to lowercase
/// 2. Replace spaces with hyphens
/// 3. Remove non-alphanumeric characters (except hyphens)
/// 4. Collapse multiple hyphens
pub fn slugify(text: &str) -> String {
    text.to_lowercase()
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
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html() {
        let md = "# Hello\n\nThis is **bold**.";
        let html = markdown_to_html(md);
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_markdown_tables() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = markdown_to_html(md);
        assert!(html.contains("<table>"));
        assert!(html.contains("<td>1</td>"));
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("API & REST: A Guide"), "api-rest-a-guide");
        assert_eq!(slugify("  Multiple   Spaces  "), "multiple-spaces");
    }

    #[test]
    fn test_extract_headings() {
        let md = "# Title\n\nContent\n\n## Section 1\n\nMore content\n\n### Subsection";
        let headings = extract_headings(md);

        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0], (1, "Title".to_string(), "title".to_string()));
        assert_eq!(
            headings[1],
            (2, "Section 1".to_string(), "section-1".to_string())
        );
        assert_eq!(
            headings[2],
            (3, "Subsection".to_string(), "subsection".to_string())
        );
    }
}
