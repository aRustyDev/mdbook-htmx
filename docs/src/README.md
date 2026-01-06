---
title: mdbook-htmx
description: HTMX-powered documentation sites with mdBook
---

# mdbook-htmx

**mdbook-htmx** is an alternative backend for [mdBook](https://rust-lang.github.io/mdBook/) that produces HTMX-enhanced documentation sites with server-side rendering capabilities, authorization support, and audience-scoped content filtering.

## What Makes It Different?

Traditional mdBook generates static HTML that works great for simple documentation. mdbook-htmx extends this with:

- **HTMX-Enhanced Navigation** - SPA-like navigation without JavaScript frameworks
- **Dual Output Mode** - Full pages for direct access, fragments for HTMX updates
- **Server Integration** - Manifest and search index for dynamic applications
- **Authorization Metadata** - Built-in support for authentication and role-based access
- **Progressive Enhancement** - Works without JavaScript, enhanced with HTMX

## Quick Start

```bash
# Install the backend
cargo install mdbook-htmx

# Add to your book.toml
[output.htmx]

# Build your documentation
mdbook build
```

Your documentation is now ready with HTMX enhancements. See the [Quick Start Guide](getting-started/quick-start.md) for a complete walkthrough.

## Who Is This For?

### Documentation Authors

If you want faster, smoother navigation without building a custom JavaScript app, mdbook-htmx gives you that out of the box. Your Markdown files work exactly as before.

### Platform Engineers

If you're building an internal documentation platform with authentication, mdbook-htmx produces the metadata you need for server-side access control. The manifest.json tells your server which pages require authentication and which roles can access them.

### Application Developers

If you're integrating documentation into a larger application, mdbook-htmx fragments can be loaded via HTMX into your existing layouts. The server model supports REST API and GraphQL integration patterns.

## Documentation Overview

| Section | Description |
|---------|-------------|
| [Concepts](concepts/README.md) | Mental models and architecture |
| [Getting Started](getting-started/README.md) | Installation and first steps |
| [Configuration](configuration/README.md) | book.toml and frontmatter options |
| [Features](features/README.md) | HTMX navigation, search, and auth |
| [Deployment](deployment/README.md) | Static, Docker, Workers, Kubernetes |
| [Tutorials](tutorials/README.md) | Step-by-step walkthroughs |
| [Reference](reference/README.md) | Complete configuration schemas |
| [ADRs](adr/README.md) | Architecture decision records |

## Development Status

mdbook-htmx is under active development. Current status:

| Stage | Status | Description |
|-------|--------|-------------|
| Core Backend | Done | Basic HTML generation |
| HTMX Integration | Done | Fragments and OOB updates |
| Authorization | Done | Frontmatter-based auth metadata |
| Search | Done | Client-side and Meilisearch support |
| Production Polish | In Progress | Documentation, testing, releases |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/aRustyDev/mdbook-htmx/blob/main/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://github.com/aRustyDev/mdbook-htmx/blob/main/LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! See the [Contributing Guide](contributing/README.md) for development setup and guidelines.
