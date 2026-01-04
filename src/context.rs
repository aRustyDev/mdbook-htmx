//! MDBook RenderContext parsing.
//!
//! MDBook passes a JSON RenderContext via stdin containing all book data.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The complete render context passed by MDBook via stdin.
#[derive(Debug, Clone, Deserialize)]
pub struct RenderContext {
    /// MDBook version string
    pub version: String,

    /// Root directory of the book source
    pub root: PathBuf,

    /// The book content and structure
    pub book: Book,

    /// Full configuration from book.toml
    pub config: BookConfig,

    /// Output destination directory
    pub destination: PathBuf,
}

/// The book content structure.
#[derive(Debug, Clone, Deserialize)]
pub struct Book {
    /// All sections (chapters) in the book
    pub sections: Vec<BookItem>,
}

/// A single item in the book (chapter or separator).
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum BookItem {
    /// A chapter with content
    Chapter(Chapter),
    /// A separator line in the TOC
    Separator,
    /// A part title (header without content)
    PartTitle(String),
}

/// A chapter in the book.
#[derive(Debug, Clone, Deserialize)]
pub struct Chapter {
    /// Chapter name/title
    pub name: String,

    /// Chapter content (Markdown)
    pub content: String,

    /// Relative path to source file
    pub path: Option<PathBuf>,

    /// Source path for the chapter
    pub source_path: Option<PathBuf>,

    /// Chapter number (e.g., [1, 2] for chapter 1.2)
    pub number: Option<Vec<u32>>,

    /// Nested sub-chapters
    #[serde(default)]
    pub sub_items: Vec<BookItem>,

    /// Parent chapter names for breadcrumb
    #[serde(default)]
    pub parent_names: Vec<String>,
}

/// The full book.toml configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct BookConfig {
    /// Book metadata
    pub book: BookMetadata,

    /// Build configuration
    #[serde(default)]
    pub build: BuildConfig,

    /// Output configurations (including [output.htmx])
    #[serde(default)]
    pub output: HashMap<String, toml::Value>,
}

/// Book metadata from `[book]` section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMetadata {
    /// Book title
    pub title: Option<String>,

    /// Book authors
    #[serde(default)]
    pub authors: Vec<String>,

    /// Book description
    pub description: Option<String>,

    /// Source directory (default: "src")
    #[serde(default = "default_src")]
    pub src: PathBuf,

    /// Language code (default: "en")
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_src() -> PathBuf {
    PathBuf::from("src")
}

fn default_language() -> String {
    "en".to_string()
}

/// Build configuration from `[build]` section.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BuildConfig {
    /// Build directory (default: "book")
    #[serde(default = "default_build_dir")]
    pub build_dir: PathBuf,

    /// Create missing files
    #[serde(default)]
    pub create_missing: bool,
}

fn default_build_dir() -> PathBuf {
    PathBuf::from("book")
}

impl RenderContext {
    /// Parse RenderContext from JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Check if MDBook version is supported (≥0.4.21).
    pub fn is_supported_version(&self) -> bool {
        if let Ok(version) = semver::Version::parse(&self.version) {
            let min_version = semver::Version::new(0, 4, 21);
            version >= min_version
        } else {
            // If we can't parse, assume it's compatible
            true
        }
    }

    /// Get all chapters (flattened from nested structure).
    pub fn iter_chapters(&self) -> impl Iterator<Item = &Chapter> {
        ChapterIterator::new(&self.book.sections)
    }
}

/// Iterator over all chapters in the book.
struct ChapterIterator<'a> {
    stack: Vec<std::slice::Iter<'a, BookItem>>,
}

impl<'a> ChapterIterator<'a> {
    fn new(sections: &'a [BookItem]) -> Self {
        Self {
            stack: vec![sections.iter()],
        }
    }
}

impl<'a> Iterator for ChapterIterator<'a> {
    type Item = &'a Chapter;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(iter) = self.stack.last_mut() {
            if let Some(item) = iter.next() {
                match item {
                    BookItem::Chapter(chapter) => {
                        if !chapter.sub_items.is_empty() {
                            self.stack.push(chapter.sub_items.iter());
                        }
                        return Some(chapter);
                    }
                    BookItem::Separator | BookItem::PartTitle(_) => continue,
                }
            } else {
                self.stack.pop();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_check() {
        let ctx = RenderContext {
            version: "0.4.40".to_string(),
            root: PathBuf::new(),
            book: Book { sections: vec![] },
            config: BookConfig {
                book: BookMetadata {
                    title: None,
                    authors: vec![],
                    description: None,
                    src: default_src(),
                    language: default_language(),
                },
                build: BuildConfig::default(),
                output: HashMap::new(),
            },
            destination: PathBuf::new(),
        };
        assert!(ctx.is_supported_version());
    }

    #[test]
    fn test_old_version() {
        let ctx = RenderContext {
            version: "0.4.0".to_string(),
            root: PathBuf::new(),
            book: Book { sections: vec![] },
            config: BookConfig {
                book: BookMetadata {
                    title: None,
                    authors: vec![],
                    description: None,
                    src: default_src(),
                    language: default_language(),
                },
                build: BuildConfig::default(),
                output: HashMap::new(),
            },
            destination: PathBuf::new(),
        };
        assert!(!ctx.is_supported_version());
    }
}
