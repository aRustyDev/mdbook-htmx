---
title: Cloudflare Workers
description: Deploy mdbook-htmx to Cloudflare's edge network
---

# Cloudflare Workers

Cloudflare Workers provide edge computing for low-latency documentation serving with optional authentication and search.

## When to Use Workers

**Advantages:**
- Global edge locations
- Sub-millisecond cold starts
- Generous free tier
- Native Cloudflare integration

**Considerations:**
- Cloudflare ecosystem required
- Different programming model
- CPU and memory limits

## Deployment Options

| Option | Description |
|--------|-------------|
| [Basic Workers](workers.md) | Static docs at the edge |
| [Workers + D1](workers-d1.md) | Add session storage with D1 |
| [Workers + Meilisearch](workers-meilisearch.md) | Secure search via Tunnel |

## Architecture

### Basic Workers

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Browser   │────▶│   Worker    │────▶│    R2/KV    │
│             │◀────│   (Edge)    │◀────│   Storage   │
└─────────────┘     └─────────────┘     └─────────────┘
```

### With D1 + Meilisearch

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Browser   │────▶│   Worker    │────▶│     D1      │
│             │◀────│   (Edge)    │     │  (Sessions) │
└─────────────┘     └──────┬──────┘     └─────────────┘
                           │
                    ┌──────▼──────┐
                    │  Cloudflare │
                    │   Tunnel    │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │ Meilisearch │
                    │  (Origin)   │
                    └─────────────┘
```

## Quick Start

```bash
# Install Wrangler CLI
npm install -g wrangler

# Deploy
wrangler deploy
```

## Next Steps

- [Basic Workers](workers.md) - Get started with Workers
- [Workers + D1](workers-d1.md) - Add authentication sessions
- [Workers + Meilisearch](workers-meilisearch.md) - Secure search integration
