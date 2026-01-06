---
title: Architecture Guide
description: Code structure and module overview
---

# Architecture Guide

Understanding the codebase structure.

## Module Overview

```
src/
├── lib.rs              # Public API
├── main.rs             # CLI entry point
├── config/
│   └── mod.rs          # HtmxConfig, parsing
├── frontmatter/
│   └── mod.rs          # YAML frontmatter
├── render/
│   ├── mod.rs          # Render orchestration
│   ├── htmx.rs         # HTMX attribute injection
│   ├── oob.rs          # Out-of-band updates
│   └── templates.rs    # Tera template handling
├── manifest/
│   └── mod.rs          # manifest.json generation
├── search/
│   └── mod.rs          # search-index.json generation
└── error.rs            # Error types
```

## Key Types

### HtmxConfig

Configuration from `[output.htmx]`:

```rust
pub struct HtmxConfig {
    pub version: String,
    pub htmx_version: String,
    pub boost: bool,
    pub swap_strategy: SwapStrategy,
    pub target: String,
    // ...
}
```

### Frontmatter

Page-level metadata:

```rust
pub struct Frontmatter {
    pub title: Option<String>,
    pub description: Option<String>,
    pub scope: Option<String>,
    pub authn: Option<String>,
    pub authz: Vec<String>,
    // ...
}
```

## Data Flow

```
1. mdBook calls backend with RenderContext
2. Parse [output.htmx] config
3. For each chapter:
   a. Extract frontmatter
   b. Render Markdown to HTML
   c. Inject HTMX attributes
   d. Generate page + fragment
   e. Add to manifest
   f. Add to search index
4. Write outputs to disk
```

## Template System

Uses Tera templates:

```
templates/
├── layout.html         # Base HTML structure
├── docs/
│   ├── page.html       # Full page wrapper
│   └── fragment.html   # Content only
└── partials/
    ├── nav.html
    ├── search.html
    └── breadcrumbs.html
```

## Adding Features

1. Add config option to `HtmxConfig`
2. Add frontmatter key if page-level
3. Implement in appropriate module
4. Update templates if needed
5. Add tests
6. Update documentation

## See Also

- [Development Setup](development.md)
- [Concepts: Architecture](../concepts/architecture.md)
