---
title: Authentication
description: Configure authentication requirements
---

# Authentication

Mark pages as requiring authentication.

## Configuration

```toml
[output.htmx.authn]
provider = "oidc"            # none, custom, oidc
signin = "/auth/login"
signout = "/auth/logout"
user-endpoint = "/api/user"
session-cookie = "session"
```

## Page-Level Settings

Set authentication requirements per page:

```yaml
---
authn: authenticated
---
```

## Authentication Levels

| Level | Meaning |
|-------|---------|
| `public` | No authentication required |
| `authenticated` | User must be logged in |
| `verified` | User must be verified |

## Manifest Integration

The manifest includes auth requirements:

```json
{
  "pages": {
    "/internal/roadmap": {
      "authn": "authenticated",
      "authn_signin": "/auth/login"
    }
  }
}
```

## Server Implementation

Your server checks auth before serving:

```python
if page.authn == "authenticated" and not user.is_authenticated:
    return redirect(page.authn_signin)
```

## See Also

- [Authorization](authorization.md) - Role-based access
- [Adding Auth Tutorial](../tutorials/adding-auth.md)
