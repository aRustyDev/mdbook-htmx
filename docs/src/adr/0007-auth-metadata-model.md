---
title: "ADR-0007: Auth Metadata Model"
description: Authentication and authorization metadata design
---

# ADR-0007: Auth Metadata Model

## Status

Accepted

## Context

mdbook-htmx needs to support access control for documentation. Key questions:

1. Where is auth metadata defined?
2. How is it propagated to outputs?
3. What granularity is supported?
4. How do servers use it?

## Decision

Implement a **metadata-only approach**:

1. **Frontmatter defines auth**: Each page specifies its requirements
2. **Manifest propagates**: Auth metadata included in manifest.json
3. **Page-level granularity**: Auth applies to entire pages
4. **Server enforces**: Actual enforcement is server responsibility

### Metadata Model

```yaml
# Frontmatter
---
scope: internal          # Audience scope
authn: authenticated     # Authentication level
authz:                   # Required roles
  - admin
  - editor
fallback: /access-denied # Redirect on denial
---
```

### Manifest Output

```json
{
  "pages": {
    "/internal/roadmap": {
      "scope": "internal",
      "authn": "authenticated",
      "authz": ["admin", "editor"],
      "fallback": "/access-denied"
    }
  }
}
```

## Consequences

### Positive

- **Simple model**: Authors understand frontmatter
- **Flexible enforcement**: Works with any auth system
- **No runtime dependency**: Build produces static metadata
- **Searchable**: Auth metadata in search index enables filtering

### Negative

- **No enforcement in build**: Protected content still generated
- **Server required**: Static hosting can't enforce
- **Trust boundary**: Clients could access fragments directly

### Mitigations

- Document that server enforcement is required
- Provide server examples for common patterns
- Search filtering respects auth in client-side mode

## Related

- [Authentication](../features/authentication.md) - Feature docs
- [Authorization](../features/authorization.md) - Role-based access
- [Manifest Schema](../reference/manifest-schema.md) - Output format
