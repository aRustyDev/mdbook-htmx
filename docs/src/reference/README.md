---
title: Reference
description: Complete reference documentation for mdbook-htmx
---

# Reference

Comprehensive reference documentation for all mdbook-htmx configuration and outputs.

## Section Contents

| Page | Description |
|------|-------------|
| [Configuration Schema](config-schema.md) | Complete `[output.htmx]` reference |
| [Frontmatter Schema](frontmatter-schema.md) | All YAML frontmatter keys |
| [Manifest Schema](manifest-schema.md) | manifest.json structure |
| [Search Index Schema](search-index-schema.md) | search-index.json structure |
| [Template API](template-api.md) | Tera template context variables |

## JSON Schemas

Machine-readable schemas are available for validation:

| Schema | URL |
|--------|-----|
| Configuration | `https://schemas.arusty.dev/mdbook-htmx/config.schema.json` |
| Manifest | `https://schemas.arusty.dev/mdbook-htmx/manifest.schema.json` |
| Search Index | `https://schemas.arusty.dev/mdbook-htmx/search-index.schema.json` |
| book.json | `https://schemas.arusty.dev/mdbook-htmx/book.schema.json` |

## Quick Reference

### Configuration

```toml
[output.htmx]
version = "1.0"
htmx-version = "1.9.10"
boost = true
swap-strategy = "innerHTML"
target = "#content"
push-url = true
output-mode = "both"
default-scope = "public"
theme-dir = "theme"

[output.htmx.navigation]
breadcrumbs = true
toc = true
prev-next = true
collapsible-sidebar = true

[output.htmx.search]
enabled = true
generate-index = true
index-content = true
heading-split-level = 3
include-auth = true

[output.htmx.assets]
hash-assets = true
sri-enabled = true

[output.htmx.authn]
provider = "none"  # none, custom, oidc
signin = "/auth/login"
signout = "/auth/logout"
session-cookie = "session"

[output.htmx.authz]
default-access = "public"  # public, authenticated, denied
default-fallback = "/access-denied"
role-claim = "roles"
strict = false
```

### Frontmatter

```yaml
---
title: Page Title
description: Meta description
scope: public|internal|...
authn: public|authenticated|verified
authz:
  - role1
  - role2
fallback: /redirect-path
template: custom-template
no_search: false
hidden: false
---
```

## Related

- [Concepts: Build Output](../concepts/build-output.md) - Understanding what's generated
- [Configuration Guide](../configuration/README.md) - Practical configuration
