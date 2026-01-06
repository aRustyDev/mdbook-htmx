---
title: "ADR-0001: Unified Server Model"
description: Single server process for all documentation features
---

# ADR-0001: Unified Server Model

## Status

Accepted

## Context

When designing the server-side integration for mdbook-htmx, we had to decide between:

1. **Split Model**: Separate processes for static files, API, search, and auth
2. **Unified Model**: Single process handling all responsibilities

Split models are common in microservices architectures, but documentation servers have different requirements than typical web applications.

## Decision

We chose the **unified server model** where a single process handles:

- Static file serving (pages, fragments, assets)
- HTMX fragment responses
- REST/GraphQL API endpoints
- Search proxy
- Authentication middleware

## Consequences

### Positive

- **Simpler deployment**: One binary, one container, one process
- **Shared state**: Manifest loaded once, auth context available everywhere
- **Consistent behavior**: Same code path for all request types
- **Easier development**: Run one process locally
- **Lower resource usage**: Single process overhead

### Negative

- **Less flexibility**: Can't scale components independently
- **Single point of failure**: One process crash affects everything
- **Technology lock-in**: Server implementation in one language

### Mitigations

- The manifest-based design allows external servers to implement the same contract
- Docker and Kubernetes enable horizontal scaling of the unified server
- The build output (manifest.json, fragments) works with any server implementation

## Related

- [Server Model](../concepts/server-model.md) - Detailed explanation
- [ADR-0002](0002-canonical-book-json.md) - Canonical data model
