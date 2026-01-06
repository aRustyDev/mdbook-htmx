---
title: book.toml Reference
description: Complete reference for [output.htmx] configuration
---

# book.toml Reference

Complete reference for the `[output.htmx]` configuration section.

## Basic Configuration

```toml
[output.htmx]
version = "1.0"              # Config schema version
htmx-version = "1.9.10"      # HTMX library version
boost = true                 # Enable hx-boost on body
swap-strategy = "innerHTML"  # Default swap method
target = "#content"          # Default swap target
push-url = true              # Update browser URL
output-mode = "both"         # full, fragments, or both
```

## Navigation

```toml
[output.htmx.navigation]
breadcrumbs = true           # Show breadcrumb trail
toc = true                   # Show table of contents
prev-next = true             # Show prev/next links
collapsible-sidebar = true   # Collapsible sections
```

## Search

```toml
[output.htmx.search]
enabled = true               # Enable search
generate-index = true        # Generate search-index.json
index-content = true         # Include body text
heading-split-level = 3      # Split at H1-H3
include-auth = true          # Include auth metadata
```

## Assets

```toml
[output.htmx.assets]
hash-assets = true           # Cache-busting hashes
sri-enabled = true           # Subresource integrity
additional-assets = "static" # Extra assets directory
```

## Authentication

```toml
[output.htmx.authn]
provider = "none"            # none, custom, oidc
signin = "/auth/login"       # Sign-in page
signout = "/auth/logout"     # Sign-out endpoint
user-endpoint = "/api/user"  # User info endpoint
session-cookie = "session"   # Session cookie name
```

## Authorization

```toml
[output.htmx.authz]
default-access = "public"    # public, authenticated, denied
default-fallback = "/access-denied"
role-claim = "roles"         # JWT claim for roles
strict = false               # Deny if claim missing
```

## See Also

- [Frontmatter](frontmatter.md) - Page-level overrides
- [Configuration Schema](../reference/config-schema.md) - JSON Schema
