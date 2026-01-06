---
title: Search
description: Full-text search configuration and integration
---

# Search

mdbook-htmx supports client-side and server-side search.

## Configuration

```toml
[output.htmx.search]
enabled = true
generate-index = true
index-content = true
heading-split-level = 3
include-auth = true
```

## Client-Side Search

Load and filter the search index in JavaScript:

```javascript
const index = await fetch('/search-index.json').then(r => r.json());

function search(query) {
  return index.documents.filter(doc =>
    doc.title.toLowerCase().includes(query.toLowerCase()) ||
    doc.body.toLowerCase().includes(query.toLowerCase())
  );
}
```

## Server-Side with Meilisearch

Import the index into Meilisearch:

```bash
curl -X POST 'http://localhost:7700/indexes/docs/documents' \
  -H 'Content-Type: application/json' \
  --data-binary @search-index.json
```

Then proxy search through your server for auth filtering.

## Search Index Structure

```json
{
  "documents": [
    {
      "id": "getting-started",
      "path": "/getting-started",
      "title": "Getting Started",
      "body": "Install mdbook-htmx...",
      "scope": "public",
      "authn": "public",
      "authz": []
    }
  ]
}
```

## Auth-Aware Filtering

When `include-auth = true`, each document includes auth metadata for filtering.

## See Also

- [Adding Search Tutorial](../tutorials/adding-search.md)
- [Search Index Schema](../reference/search-index-schema.md)
