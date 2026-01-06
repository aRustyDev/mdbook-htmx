---
title: Server Model
description: Unified server design and integration patterns for mdbook-htmx
---

# Server Model

mdbook-htmx is designed around a unified server model where a single process can serve documentation with full dynamic capabilities. This page explains the design and integration patterns.

## The Unified Server Concept

Rather than splitting functionality:

```
# Split model (not recommended)
nginx              → static files
docs-api           → REST endpoints
search-service     → Meilisearch proxy
auth-service       → authentication
```

mdbook-htmx encourages a unified approach:

```
# Unified model
single-server
├── Static files (pages, assets)
├── Fragment responses (HTMX)
├── REST/GraphQL API
├── Search proxy
└── Auth middleware
```

### Why Unified?

1. **Simpler Deployment** - One binary, one container, one process
2. **Shared State** - Manifest loaded once, auth context available everywhere
3. **Consistent Behavior** - Same code path for all request types
4. **Easier Development** - Run one process locally

## Request Flow

### Initial Page Load

```
Browser                          Server
   │                               │
   │───── GET /chapter-1 ─────────▶│
   │                               │
   │      [Check manifest.json]    │
   │      [Verify auth if needed]  │
   │      [Serve pages/chapter-1]  │
   │                               │
   │◀──── Full HTML Page ──────────│
```

### HTMX Navigation

```
Browser                          Server
   │                               │
   │── GET /fragments/chapter-2 ──▶│
   │   Header: HX-Request: true    │
   │                               │
   │      [Check manifest.json]    │
   │      [Verify auth if needed]  │
   │      [Serve fragment + OOB]   │
   │                               │
   │◀── HTML Fragment + OOB ───────│
   │                               │
   │   [Swap into #content]        │
   │   [Process OOB updates]       │
```

### Search Query

```
Browser                          Server                    Meilisearch
   │                               │                           │
   │── GET /search?q=auth ────────▶│                           │
   │   Header: HX-Request: true    │                           │
   │                               │────── Search Query ──────▶│
   │                               │◀───── Results ────────────│
   │                               │                           │
   │      [Filter by user scope]   │                           │
   │      [Render results HTML]    │                           │
   │                               │                           │
   │◀── Results HTML Fragment ─────│                           │
```

## Integration Patterns

### Pattern 1: Static + HTMX

The simplest pattern: serve static files, let HTMX handle navigation.

```
book/htmx/
├── pages/          → Serve at /
├── fragments/      → Serve at /fragments/
└── assets/         → Serve at /assets/
```

Works with any static file server. No auth, no search filtering.

### Pattern 2: Server with Auth

Add authentication by checking the manifest before serving:

```python
# Pseudocode
@app.route("/<path:path>")
def serve_page(path):
    page = manifest.pages.get(path)

    if not page:
        return 404

    if page.authn == "authenticated":
        if not current_user.is_authenticated:
            return redirect("/login")

    if page.authz:
        if not current_user.has_any_role(page.authz):
            return redirect(page.fallback or "/access-denied")

    # Serve appropriate file based on HX-Request header
    if request.headers.get("HX-Request"):
        return send_file(page.fragment_path)
    else:
        return send_file(page.page_path)
```

### Pattern 3: Dynamic Rendering

Use book.json for server-side rendering with custom templates:

```python
@app.route("/<path:path>")
def render_page(path):
    chapter = book.find_chapter(path)

    if not chapter:
        return 404

    # Custom template rendering
    if request.headers.get("HX-Request"):
        return render_template(
            "fragment.html",
            chapter=chapter,
            user=current_user
        )
    else:
        return render_template(
            "page.html",
            chapter=chapter,
            user=current_user,
            navigation=book.navigation
        )
```

### Pattern 4: API + Docs

Combine documentation with API endpoints:

```
/docs/*           → Documentation (from manifest)
/api/v1/*         → REST API
/graphql          → GraphQL endpoint
/search           → Search proxy
```

The server loads the manifest once and routes requests appropriately.

## Detecting HTMX Requests

HTMX adds headers to identify its requests:

| Header | Value | Meaning |
|--------|-------|---------|
| `HX-Request` | `true` | Request initiated by HTMX |
| `HX-Target` | Element ID | Where response will be swapped |
| `HX-Trigger` | Element ID | Element that triggered request |
| `HX-Current-URL` | URL | Current page URL |

Use these to decide what to return:

```python
if request.headers.get("HX-Request"):
    # Return fragment for HTMX
    return render_fragment(page)
else:
    # Return full page for direct access
    return render_full_page(page)
```

## Search Integration

### Client-Side Search

Load search-index.json and filter in JavaScript:

```javascript
const index = await fetch('/search-index.json').then(r => r.json());

function search(query) {
  return index.documents.filter(doc =>
    doc.title.includes(query) || doc.body.includes(query)
  );
}
```

### Server-Side with Meilisearch

Import the search index into Meilisearch:

```bash
# Import index
curl -X POST 'http://localhost:7700/indexes/docs/documents' \
  -H 'Content-Type: application/json' \
  --data-binary @search-index.json
```

Proxy search through your server for auth filtering:

```python
@app.route("/search")
def search():
    query = request.args.get("q")

    # Search Meilisearch
    results = meilisearch.index("docs").search(query)

    # Filter by user's access
    visible_results = [
        r for r in results.hits
        if user_can_access(current_user, r)
    ]

    # Return HTML fragment
    return render_template("search-results.html", results=visible_results)
```

## Deployment Considerations

### Static Hosting

When deploying to static hosts (GitHub Pages, Netlify):
- Only pages/ directory is needed
- fragments/ enables HTMX navigation
- No server-side auth filtering possible

### Container Deployment

With Docker or Kubernetes:
- Single container with embedded server
- Mount manifest.json for configuration
- Sidecar for Meilisearch if needed

### Cloudflare Workers

Edge deployment pattern:
- Workers proxy requests
- D1 for session storage
- Meilisearch via Cloudflare Tunnel

See [Deployment Guides](../deployment/README.md) for specific patterns.

## Next Steps

- [Deployment Overview](../deployment/README.md) - Deployment patterns
- [Server Integration](../advanced/server-integration.md) - Advanced patterns
- [Authentication](../features/authentication.md) - Auth configuration
