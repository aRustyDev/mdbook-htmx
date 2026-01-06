---
title: Configuration
description: Configure mdbook-htmx through book.toml and frontmatter
---

# Configuration

mdbook-htmx is configured through two mechanisms:

1. **book.toml** - Global settings in `[output.htmx]`
2. **Frontmatter** - Per-page settings in YAML

## Configuration Layers

Settings cascade from global to page-level:

```
book.toml [output.htmx]     → Global defaults
         │
         ▼
  Page frontmatter          → Page overrides
         │
         ▼
    Final settings          → Merged result
```

## Section Contents

| Page | Description |
|------|-------------|
| [book.toml Reference](book-toml.md) | All `[output.htmx]` options |
| [Frontmatter](frontmatter.md) | Page-level YAML metadata |
| [Navigation](navigation.md) | Breadcrumbs, TOC, prev/next |
| [Theming](theming.md) | CSS variables and custom templates |

## Quick Reference

### book.toml

```toml
[output.htmx]
boost = true                    # Enable hx-boost
swap-strategy = "innerHTML"     # Default swap method
target = "#content"             # Default swap target
output-mode = "both"            # pages + fragments

[output.htmx.navigation]
breadcrumbs = true
toc = true
prev-next = true

[output.htmx.search]
enabled = true
include-auth = true
```

### Frontmatter

```yaml
---
title: Custom Title
description: Page description
scope: internal
authn: authenticated
authz:
  - admin
  - editor
---
```

## Next Steps

- [book.toml Reference](book-toml.md) for complete configuration options
- [Frontmatter](frontmatter.md) for page-level settings
