---
title: Search Index Schema
description: search-index.json structure reference
---

# Search Index Schema

Reference for the `search-index.json` output file.

## Schema URL

```
https://schemas.arusty.dev/mdbook-htmx/search-index.schema.json
```

## Structure

```json
{
  "$schema": "https://schemas.arusty.dev/mdbook-htmx/search-index.schema.json",
  "version": "1.0.0",
  "config": { ... },
  "documents": [ ... ]
}
```

## Config Object

```json
{
  "heading_split_level": 3,
  "include_auth": true
}
```

## Document Object

```json
{
  "id": "getting-started",
  "path": "/getting-started",
  "title": "Getting Started",
  "body": "Install mdbook-htmx from crates.io...",
  "headings": ["Getting Started", "Installation", "Configuration"],
  "anchor": "installation",
  "scope": "public",
  "authn": "public",
  "authz": []
}
```

## Document Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique document ID |
| `path` | string | Page URL path |
| `title` | string | Document title |
| `body` | string | Content text |
| `headings` | array | Heading hierarchy |
| `anchor` | string | Section anchor (if split) |
| `scope` | string | Audience scope |
| `authn` | string | Auth requirement |
| `authz` | array | Required roles |

## Meilisearch Import

```bash
# Create index
curl -X POST 'http://localhost:7700/indexes' \
  -H 'Content-Type: application/json' \
  -d '{"uid": "docs", "primaryKey": "id"}'

# Import documents
curl -X POST 'http://localhost:7700/indexes/docs/documents' \
  -H 'Content-Type: application/json' \
  --data-binary @search-index.json
```

## See Also

- [Search Feature](../features/search.md) - Search configuration
- [Adding Search Tutorial](../tutorials/adding-search.md)
