---
title: "ADR-0003: Fragments vs Pages"
description: Dual output mode for full pages and content fragments
---

# ADR-0003: Fragments vs Pages

## Status

Accepted

## Context

HTMX navigation works by swapping content into the page. This requires:

1. **Full pages** for direct navigation (bookmarks, search engines)
2. **Fragments** for HTMX updates (just the content portion)

We needed to decide how to structure this dual output.

## Decision

Generate both output types in parallel directories:

```
book/htmx/
├── pages/              # Full HTML with layout
│   └── chapter-1.html
└── fragments/          # Content only
    └── chapter-1.html
```

Links in pages point to pages (for progressive enhancement).
HTMX attributes fetch from fragments.

## Consequences

### Positive

- **Progressive enhancement**: Works without JavaScript
- **SEO friendly**: Full pages indexable by search engines
- **Fast navigation**: Fragments are smaller, faster to load
- **Simple routing**: Parallel directory structure is predictable
- **Flexible serving**: Static or dynamic serving both work

### Negative

- **Larger output**: Duplicate content in two formats
- **Build time**: More files to generate
- **Sync complexity**: Both outputs must stay consistent

### Mitigations

- Output mode config (`full`, `fragments`, `both`)
- Build process ensures consistency
- Content hashes verify integrity

## Fragment Structure

Fragments include OOB (Out-of-Band) updates:

```html
<article id="content">
  <h1>Chapter 1</h1>
  ...
</article>

<nav id="breadcrumbs" hx-swap-oob="true">...</nav>
<nav id="toc" hx-swap-oob="true">...</nav>
```

## Related

- [HTMX Navigation](../features/htmx-navigation.md) - Navigation details
- [ADR-0006](0006-oob-updates.md) - OOB update design
