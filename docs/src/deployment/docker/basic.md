---
title: Docker Basic Setup
description: Deploy mdbook-htmx with Docker
---

# Docker Basic Setup

Run mdbook-htmx documentation in a Docker container.

## compose.yml

```yaml
services:
  docs:
    image: ghcr.io/arustydev/mdbook-htmx:latest
    volumes:
      - ./book:/app/book:ro
    ports:
      - "3000:3000"
    environment:
      - DOCS_ROOT=/app/book/htmx
```

## Build and Run

```bash
# Build your docs first
mdbook build

# Start the container
docker compose up -d

# View logs
docker compose logs -f
```

## Custom Dockerfile

If you need customization:

```dockerfile
FROM ghcr.io/arustydev/mdbook-htmx:latest

# Copy your built docs
COPY book/htmx /app/book/htmx

# Set environment
ENV DOCS_ROOT=/app/book/htmx

EXPOSE 3000
CMD ["serve"]
```

## Health Check

The container exposes a health endpoint:

```bash
curl http://localhost:3000/health
```

## See Also

- [With Search](with-search.md) - Add Meilisearch
- [Kubernetes Basic](../kubernetes/basic.md) - Container orchestration
