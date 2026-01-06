---
title: Server Integration
description: Integrate mdbook-htmx with REST API and GraphQL
---

# Server Integration

Integrate documentation with your application server.

## Loading the Manifest

```python
import json

with open('book/htmx/manifest.json') as f:
    manifest = json.load(f)
```

## REST API Pattern

Serve documentation through your API:

```python
from flask import Flask, request, send_file

app = Flask(__name__)

@app.route('/docs/<path:path>')
def serve_docs(path):
    page = manifest['pages'].get(f'/{path}')
    if not page:
        return {'error': 'Not found'}, 404

    # Check authorization
    if not can_access(current_user, page):
        return {'error': 'Forbidden'}, 403

    # Serve appropriate format
    if request.headers.get('HX-Request'):
        return send_file(page['fragment_path'])
    else:
        return send_file(page['page_path'])
```

## GraphQL Pattern

Expose documentation through GraphQL:

```graphql
type Query {
  page(path: String!): Page
  search(query: String!): [SearchResult!]!
}

type Page {
  path: String!
  title: String!
  content: String!
  scope: String
  authz: [String!]
}
```

## Embedding in Apps

Load documentation fragments into your application:

```javascript
// Load help content
htmx.ajax('GET', '/docs/fragments/help.html', '#help-panel');
```

## See Also

- [Server Model](../concepts/server-model.md) - Architecture
- [Micro-Frontends](micro-frontends.md) - Fragment embedding
