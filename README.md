# mdbook-htmx

[![CI](https://github.com/aRustyDev/mdbook-htmx/actions/workflows/ci.yml/badge.svg)](https://github.com/aRustyDev/mdbook-htmx/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/mdbook-htmx.svg)](https://crates.io/crates/mdbook-htmx)
[![License](https://img.shields.io/crates/l/mdbook-htmx.svg)](LICENSE)

An alternative MDBook backend that produces HTMX-enhanced HTML documentation with server-side rendering capabilities, authorization support, and audience-scoped content filtering.

## Features

- **HTMX-Enhanced Navigation**: SPA-like navigation without JavaScript frameworks using `hx-boost`, `hx-target`, and `hx-swap`
- **Dual Output Mode**: Generates both full HTML pages and content-only fragments for efficient HTMX updates
- **Server-Side Integration**: Produces `manifest.json` with page metadata for server-side routing and access control
- **Authorization Support**: Frontmatter-based authentication/authorization metadata for scope-aware content
- **Progressive Enhancement**: Works without JavaScript, enhanced experience with HTMX
- **Search Index Generation**: Client-side or server-side search with scope filtering

## Installation

### From crates.io

```bash
cargo install mdbook-htmx
```

### From source

```bash
git clone https://github.com/aRustyDev/mdbook-htmx
cd mdbook-htmx
cargo install --path .
```

## Usage

Add the backend to your `book.toml`:

```toml
[output.htmx]
# All options are optional with sensible defaults

# Output directory name (default: "htmx")
output_dir = "htmx"

# Enable HTMX-enhanced navigation (default: true)
htmx_enabled = true

# Generate fragments alongside full pages (default: true)
generate_fragments = true

# Generate manifest.json (default: true)
generate_manifest = true

# Default audience scope for unscoped content
default_scope = "public"
```

Build your book:

```bash
mdbook build
```

## Output Structure

```
book/
└── htmx/
    ├── pages/           # Full HTML pages
    │   ├── index.html
    │   └── chapter-1.html
    ├── fragments/       # Content-only fragments for HTMX
    │   ├── index.html
    │   └── chapter-1.html
    ├── manifest.json    # Page metadata for server integration
    └── search-index.json # Search data
```

## Frontmatter

Add YAML frontmatter to your Markdown files to control rendering:

```markdown
---
title: My Custom Title
description: Page description for meta tags
scope: internal
authn: authenticated
authz:
  - admin
  - editor
fallback: /access-denied
---

# Chapter Content
```

### Frontmatter Keys

| Key | Type | Description |
|-----|------|-------------|
| `title` | string | Override chapter title |
| `description` | string | Meta description |
| `scope` | string | Audience scope (e.g., "public", "internal") |
| `authn` | string | Auth level: "public", "authenticated", "verified" |
| `authz` | array | Required roles for access |
| `fallback` | string | Redirect path when access denied |
| `template` | string | Custom template override |
| `no_search` | bool | Exclude from search index |
| `hidden` | bool | Exclude from navigation |

## Server Integration

The `manifest.json` file contains all page metadata for server-side routing:

```json
{
  "$schema": "https://schemas.arusty.dev/mdbook-htmx/manifest.schema.json",
  "version": "1.0.0",
  "generated_at": "2024-01-01T00:00:00Z",
  "pages": {
    "/chapter-1": {
      "title": "Chapter 1",
      "source": "chapter-1.md",
      "page_path": "pages/chapter-1.html",
      "fragment_path": "fragments/chapter-1.html",
      "scope": "internal",
      "authn": "authenticated",
      "content_hash": "abc123..."
    }
  }
}
```

## Development Status

This project is under active development. See the [implementation plan](https://github.com/aRustyDev/mdbook-htmx/issues/1) for roadmap.

| Stage | Version | Status | Description |
|-------|---------|--------|-------------|
| 0 | - | ✅ | Project Bootstrap |
| 1 | v0.1.0 | 🚧 | Core Backend |
| 2 | v0.2.0 | ⏳ | HTMX Integration |
| 3 | v0.3.0 | ⏳ | Authorization |
| 4 | v0.4.0 | ⏳ | Search |
| 5 | v1.0.0 | ⏳ | Production Polish |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Contributions are welcome! Please read the [contributing guidelines](CONTRIBUTING.md) first.
