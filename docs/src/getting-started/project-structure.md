---
title: Project Structure
description: Understanding mdbook-htmx project layout
---

# Project Structure

Understanding the directory layout helps you organize your documentation effectively.

## Source Structure

```
my-docs/
├── book.toml           # Configuration
├── src/
│   ├── SUMMARY.md      # Navigation structure
│   ├── README.md       # Landing page
│   ├── chapter-1.md    # Content pages
│   └── chapter-2/
│       ├── README.md   # Section landing
│       └── topic.md    # Nested content
└── theme/              # Optional custom templates
    ├── layout.html
    └── styles.css
```

## Output Structure

After building:

```
book/htmx/
├── book.json           # Canonical data
├── manifest.json       # Server metadata
├── search-index.json   # Search data
├── assets/
│   ├── htmx.min.js
│   └── style.css
├── pages/              # Full HTML pages
│   ├── index.html
│   └── chapter-1.html
└── fragments/          # Content fragments
    ├── index.html
    └── chapter-1.html
```

## Key Files

| File | Purpose |
|------|---------|
| `book.toml` | Project configuration |
| `SUMMARY.md` | Navigation structure |
| `manifest.json` | Server integration metadata |
| `search-index.json` | Full-text search data |

## Next Steps

- [book.toml Reference](../configuration/book-toml.md) - Configuration options
- [Frontmatter](../configuration/frontmatter.md) - Page-level settings
