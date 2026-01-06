---
title: Deployment
description: Deploy mdbook-htmx to various platforms
---

# Deployment

mdbook-htmx can be deployed to various platforms, from static hosting to Kubernetes clusters. Choose the approach that fits your needs.

## Deployment Models

### Static Hosting

The simplest approach: serve pre-built HTML files from a CDN.

**Best for:**
- Public documentation
- No authentication required
- Maximum simplicity

**Options:**
- [GitHub Pages](static/github-pages.md)
- [Cloudflare Pages](static/cloudflare-pages.md)

### Container-Based

Run a server that provides dynamic features.

**Best for:**
- Authentication/authorization
- Server-side search
- API integration

**Options:**
- [Docker Basic](docker/basic.md)
- [Docker with Search](docker/with-search.md)

### Edge Computing

Deploy to edge locations for low latency.

**Best for:**
- Global audience
- Edge caching
- Cloudflare ecosystem

**Options:**
- [Cloudflare Workers](cloudflare/workers.md)
- [Workers + D1](cloudflare/workers-d1.md)
- [Workers + Meilisearch](cloudflare/workers-meilisearch.md)

### Kubernetes

Enterprise-grade deployment with orchestration.

**Best for:**
- Existing Kubernetes clusters
- Complex authentication (OAuth2 Proxy)
- High availability requirements

**Options:**
- [Basic Kubernetes](kubernetes/basic.md)
- [Helm Chart](kubernetes/helm.md)
- [Ingress Authentication](kubernetes/ingress-auth.md)
- [With Meilisearch](kubernetes/with-meilisearch.md)

## Decision Guide

| Requirement | Recommended Approach |
|-------------|---------------------|
| Public docs, no auth | Static hosting |
| Simple auth, few users | Docker or Workers |
| Complex auth, enterprise | Kubernetes + OAuth2 Proxy |
| Full-text search, private | Any server-based approach |
| Global low latency | Cloudflare Workers |
| Existing K8s cluster | Kubernetes deployment |

## Section Contents

| Section | Pages |
|---------|-------|
| [Static Hosting](static/README.md) | GitHub Pages, Cloudflare Pages |
| [Docker](docker/README.md) | Basic setup, with Meilisearch |
| [Cloudflare](cloudflare/README.md) | Workers, D1, Meilisearch proxy |
| [Kubernetes](kubernetes/README.md) | Basic, Helm, auth, search |

## Next Steps

Choose your deployment path from the sections above, or start with the simplest approach:

1. **Just getting started?** → [GitHub Pages](static/github-pages.md)
2. **Need authentication?** → [Docker Basic](docker/basic.md)
3. **Need search + auth?** → [Docker with Search](docker/with-search.md)
