---
title: Kubernetes
description: Deploy mdbook-htmx to Kubernetes clusters
---

# Kubernetes Deployment

Kubernetes provides enterprise-grade orchestration for mdbook-htmx with high availability, scaling, and advanced authentication patterns.

## When to Use Kubernetes

**Advantages:**
- High availability
- Horizontal scaling
- Enterprise authentication (OAuth2 Proxy)
- Existing infrastructure integration

**Considerations:**
- Requires Kubernetes cluster
- More complex setup
- Higher operational overhead

## Deployment Options

| Option | Description |
|--------|-------------|
| [Basic Deployment](basic.md) | Deployment, Service, Ingress |
| [Helm Chart](helm.md) | Parameterized deployment |
| [Ingress Authentication](ingress-auth.md) | OAuth2 Proxy, Basic Auth |
| [With Meilisearch](with-meilisearch.md) | Full search integration |

## Architecture

### Basic

```
┌─────────────────────────────────────────────────┐
│                 Kubernetes Cluster              │
│                                                 │
│  ┌─────────────┐    ┌─────────────┐            │
│  │   Ingress   │───▶│   Service   │            │
│  └─────────────┘    └──────┬──────┘            │
│                            │                    │
│                     ┌──────▼──────┐            │
│                     │ Deployment  │            │
│                     │  (3 pods)   │            │
│                     └─────────────┘            │
│                                                 │
└─────────────────────────────────────────────────┘
```

### With OAuth2 Proxy

```
┌─────────────────────────────────────────────────┐
│                 Kubernetes Cluster              │
│                                                 │
│  ┌─────────────┐    ┌─────────────┐            │
│  │   Ingress   │───▶│ OAuth2 Proxy│            │
│  └─────────────┘    └──────┬──────┘            │
│                            │                    │
│                     ┌──────▼──────┐            │
│                     │    Docs     │            │
│                     │   Service   │            │
│                     └─────────────┘            │
│                                                 │
└─────────────────────────────────────────────────┘
```

### With Meilisearch

```
┌─────────────────────────────────────────────────┐
│                 Kubernetes Cluster              │
│                                                 │
│  ┌─────────────┐    ┌─────────────┐            │
│  │   Ingress   │───▶│    Docs     │            │
│  └─────────────┘    └──────┬──────┘            │
│                            │                    │
│                     ┌──────▼──────┐            │
│                     │ Meilisearch │            │
│                     │   Service   │            │
│                     └─────────────┘            │
│                                                 │
└─────────────────────────────────────────────────┘
```

## Quick Start

```bash
# Apply manifests
kubectl apply -f k8s/

# Or use Helm
helm install docs ./charts/mdbook-htmx
```

## Next Steps

- [Basic Deployment](basic.md) - Simple Kubernetes setup
- [Helm Chart](helm.md) - Parameterized installation
- [Ingress Authentication](ingress-auth.md) - Add OAuth2 or Basic Auth
- [With Meilisearch](with-meilisearch.md) - Full search capabilities
