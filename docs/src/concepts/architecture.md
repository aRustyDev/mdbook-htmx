---
title: Architecture
description: System overview and data flow in mdbook-htmx
---

# Architecture

mdbook-htmx is an mdBook backend that transforms Markdown into HTMX-enhanced HTML. Understanding the architecture helps you make the most of its features.

## Build Pipeline

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Markdown   │────▶│   mdBook    │────▶│ mdbook-htmx │
│   + YAML    │     │   Parser    │     │   Backend   │
└─────────────┘     └─────────────┘     └─────────────┘
                                               │
                    ┌──────────────────────────┼──────────────────────────┐
                    │                          │                          │
                    ▼                          ▼                          ▼
             ┌─────────────┐           ┌─────────────┐           ┌─────────────┐
             │   book.json │           │    HTML     │           │   Search    │
             │  (Canonical)│           │   Output    │           │   Index     │
             └─────────────┘           └─────────────┘           └─────────────┘
                    │                          │
                    │              ┌───────────┴───────────┐
                    ▼              ▼                       ▼
             ┌─────────────┐┌─────────────┐       ┌─────────────┐
             │ manifest.json│  pages/     │       │ fragments/  │
             │             ││ Full HTML   │       │ HTML only   │
             └─────────────┘└─────────────┘       └─────────────┘
```

### Input Stage

mdBook parses your source files:
- `SUMMARY.md` defines navigation structure
- Markdown files become chapters
- YAML frontmatter provides metadata

### Processing Stage

mdbook-htmx processes the parsed content:
1. Extract frontmatter (title, scope, auth, etc.)
2. Transform Markdown to HTML
3. Inject HTMX attributes
4. Generate navigation structures
5. Build search index entries

### Output Stage

The backend produces several output types:
- **book.json** - Canonical data model
- **manifest.json** - Server integration metadata
- **pages/** - Full HTML pages with layout
- **fragments/** - Content-only HTML for HTMX
- **search-index.json** - Full-text search data

## Data Flow

### Full Page Request

```
Browser                    Server                    Files
   │                         │                         │
   │──── GET /chapter-1 ────▶│                         │
   │                         │◀──── pages/chapter-1.html
   │◀─── Full HTML Page ─────│                         │
   │                         │                         │
```

Direct navigation or first page load returns the full page with layout, CSS, and JavaScript.

### HTMX Fragment Request

```
Browser                    Server                    Files
   │                         │                         │
   │─ GET /fragments/ch1 ───▶│                         │
   │   (hx-get triggered)    │◀─ fragments/chapter-1.html
   │                         │                         │
   │◀── HTML Fragment ───────│                         │
   │   + OOB updates         │                         │
   │                         │                         │
   │ (swap into #content)    │                         │
   │                         │                         │
```

HTMX navigation fetches only the fragment, swapping it into the page.

### Search Request

```
Browser                    Server                    Search
   │                         │                         │
   │── GET /search?q=auth ──▶│                         │
   │   (hx-trigger)          │────── Query ──────────▶│
   │                         │◀───── Results ─────────│
   │◀─── Results HTML ───────│                         │
   │                         │                         │
```

Search can use client-side JavaScript with the static index, or server-side Meilisearch.

## Component Overview

### Templates (Tera)

mdbook-htmx uses [Tera](https://keats.github.io/tera/) templates:

| Template | Purpose |
|----------|---------|
| `layout.html` | Base HTML structure |
| `docs/page.html` | Full page wrapper |
| `docs/fragment.html` | Content-only output |
| `partials/nav.html` | Navigation components |
| `partials/search.html` | Search interface |

### Configuration Layers

Configuration cascades from global to page-level:

```
book.toml [output.htmx]     (Global defaults)
         │
         ▼
  Page frontmatter          (Page overrides)
         │
         ▼
    Final settings          (Merged result)
```

### File Outputs

```
book/htmx/
├── book.json              # Canonical book data
├── manifest.json          # Server integration
├── search-index.json      # Full-text search
├── assets/                # CSS, JS, images
│   ├── htmx.min.js
│   ├── style.css
│   └── ...
├── pages/                 # Full HTML pages
│   ├── index.html
│   ├── concepts/
│   │   ├── architecture.html
│   │   └── ...
│   └── ...
└── fragments/             # Content fragments
    ├── index.html
    ├── concepts/
    │   ├── architecture.html
    │   └── ...
    └── ...
```

## Integration Points

### Static Hosting

For static hosting (GitHub Pages, Cloudflare Pages), serve the `pages/` directory. HTMX enhancements work with the `fragments/` directory at the same origin.

### Dynamic Server

For server integration, use:
- `manifest.json` for routing and access control
- `book.json` for custom rendering
- `search-index.json` for search features

### API Integration

The manifest provides paths for building REST or GraphQL APIs:

```json
{
  "pages": {
    "/concepts/architecture": {
      "fragment_path": "fragments/concepts/architecture.html",
      "authz": ["viewer", "admin"]
    }
  }
}
```

Your server can check authorization before serving fragments.

## Next Steps

- [Build Output](build-output.md) - Detailed explanation of output files
- [Server Model](server-model.md) - Server integration patterns
