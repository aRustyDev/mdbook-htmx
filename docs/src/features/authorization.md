---
title: Authorization
description: Role-based access control
---

# Authorization

Control page access with roles and scopes.

## Configuration

```toml
[output.htmx.authz]
default-access = "public"
default-fallback = "/access-denied"
role-claim = "roles"
strict = false
```

## Page-Level Authorization

```yaml
---
authz:
  - admin
  - editor
fallback: /request-access
---
```

## Scopes

Audience-based content filtering:

```yaml
---
scope: internal
---
```

Common scopes:
- `public` - Available to everyone
- `internal` - Internal users only
- `partner` - Partner organizations

## Manifest Integration

```json
{
  "pages": {
    "/internal/roadmap": {
      "scope": "internal",
      "authz": ["staff", "admin"],
      "fallback": "/access-denied"
    }
  }
}
```

## Server Implementation

```python
def can_access(user, page):
    if page.authz and not any(role in user.roles for role in page.authz):
        return False, page.fallback
    return True, None
```

## Strict Mode

When `strict = true`, deny access if role claim is missing.

## See Also

- [Authentication](authentication.md) - Login requirements
- [Auth Metadata ADR](../adr/0007-auth-metadata-model.md)
