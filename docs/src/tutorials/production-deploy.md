---
title: Production Deployment
description: Deploy documentation to production
---

# Production Deployment

Deploy your documentation site to production.

## Goal

Set up a production-ready deployment with:
- CI/CD pipeline
- Search integration
- Monitoring

## Prerequisites

- Completed tutorials
- GitHub repository
- Deployment target (choose one)

## Step 1: Set Up GitHub Actions

Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy Docs

on:
  push:
    branches: [main]

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Install mdbook-htmx
        run: cargo install mdbook-htmx

      - name: Build
        run: mdbook build

      - name: Deploy
        # Add your deployment step
```

## Step 2: Configure Secrets

Add secrets in GitHub Settings:

- `MEILISEARCH_URL`
- `MEILISEARCH_KEY`
- Platform-specific secrets

## Step 3: Update Search on Deploy

Add to workflow:

```yaml
- name: Update search index
  run: |
    curl -X POST "$MEILISEARCH_URL/indexes/docs/documents" \
      -H "Authorization: Bearer $MEILISEARCH_KEY" \
      --data-binary @book/htmx/search-index.json
```

## Step 4: Add Monitoring

- Set up uptime monitoring
- Configure error tracking
- Add analytics (privacy-respecting)

## Verification

- [ ] Builds succeed on push
- [ ] Site is accessible
- [ ] Search works
- [ ] Auth works (if enabled)

## Deployment Options

Choose your platform:

- [GitHub Pages](../deployment/static/github-pages.md)
- [Docker](../deployment/docker/basic.md)
- [Cloudflare Workers](../deployment/cloudflare/workers.md)
- [Kubernetes](../deployment/kubernetes/basic.md)
