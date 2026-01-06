---
title: Adding Search
description: Integrate Meilisearch for full-text search
---

# Adding Search

Add server-side search with Meilisearch.

## Goal

Implement full-text search with:
- Real-time results
- Auth-aware filtering
- HTMX integration

## Prerequisites

- Completed [first docs site](first-docs-site.md)
- Docker installed

## Step 1: Start Meilisearch

```bash
docker run -d \
  --name meilisearch \
  -p 7700:7700 \
  -e MEILI_MASTER_KEY=your-secret-key \
  getmeili/meilisearch:v1.6
```

## Step 2: Import Search Index

```bash
curl -X POST 'http://localhost:7700/indexes/docs/documents' \
  -H 'Authorization: Bearer your-secret-key' \
  -H 'Content-Type: application/json' \
  --data-binary @book/htmx/search-index.json
```

## Step 3: Configure Search Settings

```bash
curl -X PATCH 'http://localhost:7700/indexes/docs/settings' \
  -H 'Authorization: Bearer your-secret-key' \
  -H 'Content-Type: application/json' \
  -d '{
    "searchableAttributes": ["title", "body", "headings"],
    "filterableAttributes": ["scope", "authn", "authz"]
  }'
```

## Step 4: Add Search Proxy

Create a simple server that proxies search requests:

```python
from flask import Flask, request, jsonify
import requests

app = Flask(__name__)

@app.route('/search')
def search():
    query = request.args.get('q')
    response = requests.post(
        'http://localhost:7700/indexes/docs/search',
        headers={'Authorization': 'Bearer your-secret-key'},
        json={'q': query}
    )
    return jsonify(response.json())
```

## Verification

- [ ] Search returns results
- [ ] Results match query
- [ ] Auth filtering works

## Next Steps

- [Adding Authentication](adding-auth.md)
- [Production Deployment](production-deploy.md)
