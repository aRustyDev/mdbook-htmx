---
title: "ADR-0004: Search Index Design"
description: Search index structure and split strategy
---

# ADR-0004: Search Index Design

## Status

Accepted

## Context

Full-text search requires an index. Key decisions:

1. What granularity? (page, section, paragraph)
2. What metadata to include?
3. How to handle auth-protected content?
4. Client-side or server-side search?

## Decision

Generate `search-index.json` with:

1. **Heading-level granularity**: Split at configurable heading level (default H3)
2. **Auth metadata included**: Scope, authn, authz per document
3. **Dual-use design**: Works for both client and server search

### Structure

```json
{
  "documents": [{
    "id": "getting-started#installation",
    "path": "/getting-started",
    "anchor": "installation",
    "title": "Installation",
    "body": "Install from crates.io...",
    "headings": ["Getting Started", "Installation"],
    "scope": "public",
    "authn": "public",
    "authz": []
  }]
}
```

## Consequences

### Positive

- **Precise results**: Heading-level splits enable deep linking
- **Auth filtering**: Server can filter by user permissions
- **Flexible use**: Same index for client or Meilisearch
- **Configurable depth**: Adjust granularity per project

### Negative

- **Larger index**: More documents than page-level
- **Complexity**: Heading parsing adds build time
- **Duplicate text**: Headings appear in multiple documents

### Mitigations

- `heading-split-level` config to control granularity
- `max-excerpt-length` to limit body size
- `include-auth` option to omit auth metadata

## Related

- [Search Feature](../features/search.md) - Configuration
- [Search Index Schema](../reference/search-index-schema.md) - Full schema
