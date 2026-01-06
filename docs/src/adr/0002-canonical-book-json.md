---
title: "ADR-0002: Canonical book.json"
description: JSON as the source of truth for documentation data
---

# ADR-0002: Canonical book.json

## Status

Accepted

## Context

Traditional static site generators produce HTML as the primary output. This creates challenges:

1. HTML is difficult to transform programmatically
2. Server-side customization requires re-parsing HTML
3. Multiple output formats require multiple build processes
4. API responses need separate data extraction

## Decision

We produce `book.json` as the **canonical data model**. HTML files (pages, fragments) are derived artifacts that can be regenerated from this data.

```
book.json (canonical) → HTML (derived)
                      → manifest.json (derived)
                      → search-index.json (derived)
```

## Consequences

### Positive

- **Server-side rendering**: Servers can use book.json with custom templates
- **Multiple formats**: Generate PDF, EPUB from same source
- **API responses**: Return chapter data as JSON directly
- **Custom builds**: Transform data without re-parsing Markdown
- **Consistent metadata**: All derived outputs share same source

### Negative

- **Larger output**: book.json adds to total size
- **Complexity**: Additional artifact to understand
- **Sync risk**: Derived files could drift from canonical

### Mitigations

- book.json is optional for simple static deployments
- Build process ensures derived files are always in sync
- Content hashes enable cache validation

## Example

```json
{
  "chapters": [{
    "path": "/getting-started",
    "title": "Getting Started",
    "content": "<p>HTML content...</p>",
    "frontmatter": { "scope": "public" }
  }]
}
```

## Related

- [Build Output](../concepts/build-output.md) - Output structure
- [ADR-0003](0003-fragments-vs-pages.md) - Dual output approach
