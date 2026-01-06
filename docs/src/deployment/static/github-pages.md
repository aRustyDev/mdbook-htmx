---
title: GitHub Pages
description: Deploy mdbook-htmx to GitHub Pages
---

# GitHub Pages

Deploy your documentation to GitHub Pages for free hosting.

## Prerequisites

- GitHub repository
- GitHub Actions enabled

## GitHub Actions Workflow

Create `.github/workflows/docs.yml`:

```yaml
name: Deploy Docs

on:
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Install mdbook-htmx
        run: cargo install mdbook-htmx

      - name: Build docs
        run: mdbook build

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: book/htmx/pages

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

## Repository Settings

1. Go to Settings > Pages
2. Source: GitHub Actions

## Custom Domain

Add a `CNAME` file to `src/`:

```
docs.example.com
```

## See Also

- [Cloudflare Pages](cloudflare-pages.md) - Alternative static hosting
- [CI/CD Pipelines](../../advanced/cicd.md) - Advanced workflows
