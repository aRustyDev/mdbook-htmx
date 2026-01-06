---
title: Build Output
description: Understanding what mdbook-htmx produces and how to use it
---

# Build Output

mdbook-htmx produces several output files, each serving a specific purpose. Understanding these helps you integrate with servers and customize behavior.

## Output Directory Structure

```
book/htmx/
├── book.json              # Canonical data model
├── manifest.json          # Server integration metadata
├── search-index.json      # Full-text search index
├── assets/
│   ├── htmx.min.js        # HTMX library
│   ├── style.css          # Theme styles
│   └── ...
├── pages/
│   ├── index.html
│   └── chapter-1.html
└── fragments/
    ├── index.html
    └── chapter-1.html
```

## book.json (Canonical Model)

The `book.json` file is the canonical representation of your documentation. HTML files are derived from this data.

```json
{
  "$schema": "https://schemas.arusty.dev/mdbook-htmx/book.schema.json",
  "version": "1.0.0",
  "title": "My Documentation",
  "description": "Project documentation",
  "chapters": [
    {
      "path": "/getting-started",
      "title": "Getting Started",
      "content": "<p>Markdown converted to HTML...</p>",
      "frontmatter": {
        "scope": "public",
        "authn": "public"
      },
      "headings": [
        {"level": 1, "text": "Getting Started", "id": "getting-started"},
        {"level": 2, "text": "Installation", "id": "installation"}
      ],
      "children": []
    }
  ],
  "navigation": {
    "toc": [...],
    "breadcrumbs": {...}
  }
}
```

### Why Canonical?

The book.json approach enables:

1. **Server-Side Rendering** - Your server can use book.json with custom templates
2. **Multiple Formats** - Generate PDF, EPUB, or other formats from one source
3. **API Responses** - Return chapter data as JSON for API consumers
4. **Custom Builds** - Transform the data without re-parsing Markdown

## manifest.json (Server Integration)

The manifest provides metadata for server-side routing and access control:

```json
{
  "$schema": "https://schemas.arusty.dev/mdbook-htmx/manifest.schema.json",
  "version": "1.0.0",
  "generated_at": "2024-01-15T10:30:00Z",
  "config": {
    "htmx_version": "1.9.10",
    "boost": true,
    "target": "#content"
  },
  "pages": {
    "/getting-started": {
      "title": "Getting Started",
      "source": "getting-started.md",
      "page_path": "pages/getting-started.html",
      "fragment_path": "fragments/getting-started.html",
      "scope": "public",
      "authn": "public",
      "authz": [],
      "fallback": null,
      "content_hash": "sha256:abc123...",
      "last_modified": "2024-01-15T10:30:00Z"
    },
    "/internal/roadmap": {
      "title": "Roadmap",
      "source": "internal/roadmap.md",
      "page_path": "pages/internal/roadmap.html",
      "fragment_path": "fragments/internal/roadmap.html",
      "scope": "internal",
      "authn": "authenticated",
      "authz": ["staff", "admin"],
      "fallback": "/access-denied",
      "content_hash": "sha256:def456...",
      "last_modified": "2024-01-14T15:00:00Z"
    }
  },
  "assets": {
    "css/style.css": {
      "hash": "sha256:...",
      "sri": "sha256-..."
    }
  }
}
```

### Using the Manifest

Server-side code can use the manifest for:

```python
# Python example
import json

manifest = json.load(open("book/htmx/manifest.json"))

def can_access(user, path):
    page = manifest["pages"].get(path)
    if not page:
        return False

    # Check authentication
    if page["authn"] == "authenticated" and not user.is_authenticated:
        return False

    # Check authorization
    if page["authz"] and not any(role in user.roles for role in page["authz"]):
        return False

    return True

def get_fragment(path):
    page = manifest["pages"].get(path)
    if page:
        return read_file(page["fragment_path"])
    return None
```

## search-index.json

The search index contains tokenized content for full-text search:

```json
{
  "$schema": "https://schemas.arusty.dev/mdbook-htmx/search-index.schema.json",
  "version": "1.0.0",
  "config": {
    "heading_split_level": 3,
    "include_auth": true
  },
  "documents": [
    {
      "id": "getting-started",
      "path": "/getting-started",
      "title": "Getting Started",
      "body": "Install mdbook-htmx from crates.io...",
      "headings": ["Getting Started", "Installation", "Configuration"],
      "scope": "public",
      "authn": "public",
      "authz": []
    }
  ]
}
```

### Search Integration Options

1. **Client-Side** - Load the index in JavaScript, filter results locally
2. **Meilisearch** - Import the index into Meilisearch for server-side search
3. **Custom** - Use the index with any search backend

## pages/ (Full HTML)

Full HTML pages include the complete layout:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <title>Getting Started - My Docs</title>
  <link rel="stylesheet" href="/assets/style.css">
  <script src="/assets/htmx.min.js"></script>
</head>
<body hx-boost="true">
  <nav id="sidebar">...</nav>
  <main>
    <nav id="breadcrumbs">...</nav>
    <article id="content">
      <h1>Getting Started</h1>
      ...
    </article>
    <nav id="page-nav">...</nav>
  </main>
</body>
</html>
```

Use pages for:
- Direct URL access
- Search engine indexing
- Non-JavaScript browsers
- Initial page load

## fragments/ (Content Only)

Fragments contain only the content portion:

```html
<article id="content">
  <h1>Getting Started</h1>
  <p>Install mdbook-htmx from crates.io...</p>
</article>

<!-- OOB updates -->
<nav id="breadcrumbs" hx-swap-oob="true">
  <a href="/">Home</a> &gt; Getting Started
</nav>

<nav id="page-nav" hx-swap-oob="true">
  <a href="/installation">Next: Installation</a>
</nav>
```

Use fragments for:
- HTMX navigation (swapping content)
- Embedding in external applications
- API responses returning HTML

## Content Hashes

Each page includes a content hash for:
- **Cache Invalidation** - Know when content changed
- **ETags** - HTTP conditional requests
- **Verification** - Ensure content integrity

```json
{
  "content_hash": "sha256:abc123def456..."
}
```

## Output Mode Options

Configure which outputs to generate:

```toml
[output.htmx]
output-mode = "both"  # "full", "fragments", or "both"
```

| Mode | pages/ | fragments/ | Use Case |
|------|--------|------------|----------|
| `full` | Yes | No | Static hosting only |
| `fragments` | No | Yes | Server-side rendering |
| `both` | Yes | Yes | Hybrid approach |

## Next Steps

- [Server Model](server-model.md) - How to integrate these outputs with a server
- [Configuration Reference](../reference/config-schema.md) - All output options
