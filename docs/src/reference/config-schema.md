---
title: Configuration Schema
description: Complete [output.htmx] configuration reference
---

# Configuration Schema

Complete reference for `[output.htmx]` configuration.

## Schema URL

```
https://schemas.arusty.dev/mdbook-htmx/config.schema.json
```

## Root Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `version` | string | `"1.0"` | Config schema version |
| `htmx-version` | string | `"1.9.10"` | HTMX library version |
| `boost` | boolean | `true` | Enable hx-boost on body |
| `swap-strategy` | string | `"innerHTML"` | Default swap method |
| `target` | string | `"#content"` | Default swap target |
| `push-url` | boolean | `true` | Update browser URL |
| `output-mode` | string | `"both"` | Output mode |
| `default-scope` | string | `null` | Default audience scope |
| `theme-dir` | string | `null` | Custom theme directory |

## Swap Strategies

| Value | Description |
|-------|-------------|
| `innerHTML` | Replace inner content |
| `outerHTML` | Replace entire element |
| `beforebegin` | Insert before element |
| `afterbegin` | Insert at start of element |
| `beforeend` | Insert at end of element |
| `afterend` | Insert after element |
| `delete` | Delete element |
| `none` | No swap |

## Output Modes

| Value | pages/ | fragments/ |
|-------|--------|------------|
| `full` | Yes | No |
| `fragments` | No | Yes |
| `both` | Yes | Yes |

## Navigation Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `breadcrumbs` | boolean | `true` | Show breadcrumbs |
| `toc` | boolean | `true` | Show table of contents |
| `prev-next` | boolean | `true` | Show prev/next links |
| `collapsible-sidebar` | boolean | `true` | Collapsible sections |

## Search Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable search |
| `generate-index` | boolean | `true` | Generate search-index.json |
| `index-content` | boolean | `true` | Include body text |
| `heading-split-level` | number | `3` | Split at heading level |
| `max-excerpt-length` | number | `null` | Max excerpt length |
| `include-auth` | boolean | `true` | Include auth metadata |

## Assets Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `hash-assets` | boolean | `true` | Hash for cache busting |
| `sri-enabled` | boolean | `true` | Subresource integrity |
| `additional-assets` | string | `null` | Extra assets directory |

## Authentication Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `provider` | string | `"none"` | Auth provider type |
| `signin` | string | `"/auth/login"` | Sign-in path |
| `signout` | string | `"/auth/logout"` | Sign-out path |
| `user-endpoint` | string | `null` | User info endpoint |
| `session-cookie` | string | `"session"` | Session cookie name |

## Authorization Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `default-access` | string | `"public"` | Default access level |
| `default-fallback` | string | `"/access-denied"` | Default fallback path |
| `role-claim` | string | `"roles"` | JWT role claim |
| `strict` | boolean | `false` | Strict mode |

## See Also

- [book.toml Reference](../configuration/book-toml.md) - Practical guide
- [Frontmatter Schema](frontmatter-schema.md) - Page-level settings
