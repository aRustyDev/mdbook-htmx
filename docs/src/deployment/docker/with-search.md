---
title: Docker with Search
description: Add Meilisearch to Docker deployment
---

# Docker with Search

Add server-side search with Meilisearch.

## compose.yml

```yaml
services:
  docs:
    image: ghcr.io/arustydev/mdbook-htmx:latest
    volumes:
      - ./book:/app/book:ro
    ports:
      - "3000:3000"
    environment:
      - DOCS_ROOT=/app/book/htmx
      - MEILISEARCH_URL=http://meilisearch:7700
      - MEILISEARCH_KEY=${MEILISEARCH_KEY}
    depends_on:
      - meilisearch

  meilisearch:
    image: getmeili/meilisearch:v1.6
    volumes:
      - meili_data:/meili_data
    environment:
      - MEILI_MASTER_KEY=${MEILISEARCH_KEY}
    ports:
      - "7700:7700"

volumes:
  meili_data:
```

## Initialize Search Index

After starting:

```bash
# Import search index
curl -X POST 'http://localhost:7700/indexes/docs/documents' \
  -H 'Authorization: Bearer ${MEILISEARCH_KEY}' \
  -H 'Content-Type: application/json' \
  --data-binary @book/htmx/search-index.json
```

## Update Index on Deploy

Add to your CI/CD:

```yaml
- name: Update search index
  run: |
    curl -X DELETE "http://meilisearch:7700/indexes/docs"
    curl -X POST "http://meilisearch:7700/indexes/docs/documents" \
      -H "Authorization: Bearer $MEILISEARCH_KEY" \
      --data-binary @book/htmx/search-index.json
```

## See Also

- [Search Feature](../../features/search.md) - Search configuration
- [Adding Search Tutorial](../../tutorials/adding-search.md)
