---
title: Docker Deployment
description: Deploy mdbook-htmx with Docker containers
---

# Docker Deployment

Docker provides a consistent, portable way to deploy mdbook-htmx with dynamic features like authentication and server-side search.

## When to Use Docker

**Advantages:**
- Consistent environment
- Easy local development
- Works with any container platform
- Full feature support

**Considerations:**
- Requires container hosting
- More setup than static hosting

## Deployment Options

| Option | Description |
|--------|-------------|
| [Basic Setup](basic.md) | Single container with docs server |
| [With Search](with-search.md) | Add Meilisearch for server-side search |

## Quick Start

```yaml
# compose.yml
services:
  docs:
    image: ghcr.io/arustydev/mdbook-htmx:latest
    volumes:
      - ./book:/app/book:ro
    ports:
      - "3000:3000"
```

```bash
docker compose up
```

## Architecture

### Basic Setup

```
┌─────────────────────────────────────┐
│           Docker Host               │
│                                     │
│  ┌─────────────────────────────┐   │
│  │      docs container         │   │
│  │                             │   │
│  │  ┌─────────┐ ┌───────────┐ │   │
│  │  │ Static  │ │  Auth     │ │   │
│  │  │ Server  │ │ Middleware│ │   │
│  │  └─────────┘ └───────────┘ │   │
│  │                             │   │
│  └─────────────────────────────┘   │
│                                     │
└─────────────────────────────────────┘
```

### With Meilisearch

```
┌─────────────────────────────────────┐
│           Docker Host               │
│                                     │
│  ┌──────────────┐ ┌──────────────┐ │
│  │    docs      │ │  meilisearch │ │
│  │  container   │◀│   container  │ │
│  └──────────────┘ └──────────────┘ │
│                                     │
└─────────────────────────────────────┘
```

## Next Steps

- [Basic Setup](basic.md) - Get started with Docker
- [With Search](with-search.md) - Add Meilisearch
