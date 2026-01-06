---
title: Cloudflare Pages
description: Deploy mdbook-htmx to Cloudflare Pages
---

# Cloudflare Pages

Deploy to Cloudflare's global CDN with Cloudflare Pages.

## Prerequisites

- Cloudflare account
- GitHub repository connected

## Setup

1. Go to Cloudflare Dashboard > Pages
2. Connect your GitHub repository
3. Configure build settings:

| Setting | Value |
|---------|-------|
| Build command | `cargo install mdbook-htmx && mdbook build` |
| Build output | `book/htmx/pages` |

## Environment Variables

Add these in Pages settings:

```
CARGO_HOME=/opt/buildhome/.cargo
```

## wrangler.toml (Optional)

For more control, add `wrangler.toml`:

```toml
name = "my-docs"
compatibility_date = "2024-01-01"

[build]
command = "cargo install mdbook-htmx && mdbook build"

[site]
bucket = "book/htmx/pages"
```

## Custom Domain

1. Add custom domain in Pages settings
2. Update DNS to point to Pages

## See Also

- [GitHub Pages](github-pages.md) - Alternative hosting
- [Cloudflare Workers](../cloudflare/workers.md) - Dynamic features
