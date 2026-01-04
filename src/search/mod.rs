//! Search index generation.
//!
//! Generates search-index.json for client-side or server-side search.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The search index containing all searchable content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndex {
    /// Schema version
    #[serde(rename = "$schema")]
    pub schema: String,

    /// Index format version
    pub version: String,

    /// Indexed documents
    pub documents: Vec<SearchDocument>,

    /// Heading anchors for deep linking
    pub headings: HashMap<String, Vec<HeadingEntry>>,
}

/// A searchable document entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocument {
    /// Document ID (URL path)
    pub id: String,

    /// Page title
    pub title: String,

    /// Plain text content for search
    pub body: String,

    /// Audience scope (for filtering)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// Breadcrumb path
    pub breadcrumbs: Vec<String>,
}

/// A heading entry for anchor linking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingEntry {
    /// Heading text
    pub text: String,

    /// Anchor ID
    pub anchor: String,

    /// Heading level (1-6)
    pub level: u8,
}

impl SearchIndex {
    /// Create a new empty search index.
    pub fn new() -> Self {
        Self {
            schema: "https://schemas.arusty.dev/mdbook-htmx/search-index.schema.json".to_string(),
            version: "1.0.0".to_string(),
            documents: Vec::new(),
            headings: HashMap::new(),
        }
    }

    /// Add a document to the index.
    pub fn add_document(&mut self, doc: SearchDocument, headings: Vec<HeadingEntry>) {
        let id = doc.id.clone();
        self.documents.push(doc);
        if !headings.is_empty() {
            self.headings.insert(id, headings);
        }
    }

    /// Serialize the index to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Filter index by scope.
    ///
    /// Returns a new index containing only documents matching the given scope.
    pub fn filter_by_scope(&self, scope: &str) -> Self {
        let documents: Vec<_> = self
            .documents
            .iter()
            .filter(|doc| doc.scope.as_deref() == Some(scope) || doc.scope.is_none())
            .cloned()
            .collect();

        let doc_ids: std::collections::HashSet<_> = documents.iter().map(|d| &d.id).collect();

        let headings: HashMap<_, _> = self
            .headings
            .iter()
            .filter(|(id, _)| doc_ids.contains(id))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        Self {
            schema: self.schema.clone(),
            version: self.version.clone(),
            documents,
            headings,
        }
    }
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new()
    }
}
