---
title: Frontmatter
description: Page-level YAML metadata configuration
---

# Frontmatter

Add YAML frontmatter to Markdown files to control page-level settings.

## Basic Syntax

```markdown
---
title: Custom Page Title
description: Meta description for SEO
---

# Page Content

Your markdown content here.
```

## Available Keys

| Key | Type | Description |
|-----|------|-------------|
| `title` | string | Override chapter title |
| `description` | string | Meta description |
| `scope` | string | Audience scope |
| `authn` | string | Auth requirement |
| `authz` | array | Required roles |
| `fallback` | string | Access denied redirect |
| `template` | string | Custom template |
| `no_search` | bool | Exclude from search |
| `hidden` | bool | Hide from navigation |

## Authentication Levels

```yaml
---
authn: public        # No authentication required
authn: authenticated # Must be logged in
authn: verified      # Must be verified user
---
```

## Authorization Roles

```yaml
---
authz:
  - admin
  - editor
fallback: /request-access
---
```

## Scope-Based Content

```yaml
---
scope: internal      # Only for internal users
scope: public        # Available to everyone
scope: partner       # For partner organizations
---
```

## Hiding Pages

```yaml
---
hidden: true         # Hide from navigation
no_search: true      # Exclude from search
---
```

## See Also

- [book.toml Reference](book-toml.md) - Global defaults
- [Frontmatter Schema](../reference/frontmatter-schema.md) - Complete schema
