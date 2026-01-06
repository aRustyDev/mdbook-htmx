---
title: Your First Docs Site
description: Create an HTMX-enhanced documentation site from scratch
---

# Your First Docs Site

Build a complete documentation site in 15 minutes.

## Goal

Create a documentation site with:
- HTMX-enhanced navigation
- Table of contents
- Search functionality

## Prerequisites

- Rust installed
- mdbook-htmx installed

## Step 1: Create Project Structure

```bash
mkdir my-docs && cd my-docs
mkdir -p src/guide src/reference
```

## Step 2: Configure book.toml

```toml
[book]
title = "My Project"
authors = ["Your Name"]
language = "en"

[output.htmx]
boost = true

[output.htmx.navigation]
breadcrumbs = true
toc = true
prev-next = true

[output.htmx.search]
enabled = true
```

## Step 3: Create Navigation

Create `src/SUMMARY.md`:

```markdown
# Summary

[Introduction](README.md)

# Guide

- [Getting Started](guide/getting-started.md)
- [Configuration](guide/configuration.md)
- [Advanced Usage](guide/advanced.md)

# Reference

- [API](reference/api.md)
- [CLI](reference/cli.md)
```

## Step 4: Write Content

Create pages referenced in SUMMARY.md with meaningful content.

## Step 5: Build

```bash
mdbook build
```

## Step 6: Preview

```bash
python -m http.server 8000 --directory book/htmx/pages
```

Open http://localhost:8000

## Verification

- [ ] Pages load instantly via HTMX
- [ ] Breadcrumbs update on navigation
- [ ] TOC shows current page headings
- [ ] Search finds content

## Next Steps

- [Adding Search](adding-search.md) - Integrate Meilisearch
- [Production Deployment](production-deploy.md) - Go live
