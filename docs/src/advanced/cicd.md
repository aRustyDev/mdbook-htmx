---
title: CI/CD Pipelines
description: Automate documentation builds and deployments
---

# CI/CD Pipelines

Automate your documentation workflow.

## GitHub Actions

```yaml
name: Docs

on:
  push:
    branches: [main]
    paths:
      - 'docs/**'
      - 'src/**'

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install mdbook-htmx
        run: cargo install mdbook-htmx

      - name: Build
        run: mdbook build

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: docs
          path: book/htmx/

  deploy:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - name: Deploy
        # Deploy to your platform
```

## Update Search Index

```yaml
update-search:
  needs: deploy
  runs-on: ubuntu-latest
  steps:
    - uses: actions/download-artifact@v4
      with:
        name: docs

    - name: Update Meilisearch
      run: |
        curl -X DELETE "$MEILISEARCH_URL/indexes/docs"
        curl -X POST "$MEILISEARCH_URL/indexes/docs/documents" \
          -H "Authorization: Bearer $MEILISEARCH_KEY" \
          --data-binary @search-index.json
```

## Preview Deployments

Deploy previews for pull requests:

```yaml
on:
  pull_request:
    paths:
      - 'docs/**'

jobs:
  preview:
    runs-on: ubuntu-latest
    steps:
      - name: Deploy preview
        # Deploy to preview URL
```

## See Also

- [GitHub Pages](../deployment/static/github-pages.md)
- [Production Deployment Tutorial](../tutorials/production-deploy.md)
