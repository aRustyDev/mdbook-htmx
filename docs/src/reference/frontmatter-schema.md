---
title: Frontmatter Schema
description: Complete frontmatter YAML reference
---

# Frontmatter Schema

Complete reference for page-level YAML frontmatter.

## Schema URL

```
https://schemas.arusty.dev/mdbook-htmx/frontmatter.schema.json
```

## All Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `title` | string | Chapter title | Page title override |
| `description` | string | `null` | Meta description |
| `scope` | string | Global default | Audience scope |
| `authn` | string | `"public"` | Authentication level |
| `authz` | array | `[]` | Required roles |
| `fallback` | string | Global default | Access denied redirect |
| `template` | string | `null` | Custom template name |
| `no_search` | boolean | `false` | Exclude from search |
| `hidden` | boolean | `false` | Hide from navigation |

## Authentication Levels

| Value | Description |
|-------|-------------|
| `public` | No authentication required |
| `authenticated` | Must be logged in |
| `verified` | Must be verified user |

## Example: Public Page

```yaml
---
title: Getting Started
description: Learn how to get started with mdbook-htmx
---
```

## Example: Protected Page

```yaml
---
title: Internal Roadmap
description: Product roadmap for internal use
scope: internal
authn: authenticated
authz:
  - staff
  - admin
fallback: /access-denied
---
```

## Example: Hidden Page

```yaml
---
title: Draft Content
hidden: true
no_search: true
---
```

## Example: Custom Template

```yaml
---
title: Special Page
template: landing
---
```

## See Also

- [Frontmatter Guide](../configuration/frontmatter.md) - Practical usage
- [Configuration Schema](config-schema.md) - Global settings
