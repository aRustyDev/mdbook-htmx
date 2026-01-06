---
title: Adding Authentication
description: Add login requirements to pages
---

# Adding Authentication

Configure authentication for protected documentation.

## Goal

Implement authentication with:
- Protected pages
- Role-based access
- Login redirect

## Prerequisites

- Completed [first docs site](first-docs-site.md)
- Auth provider (OAuth, OIDC, etc.)

## Step 1: Configure Authentication

Update `book.toml`:

```toml
[output.htmx.authn]
provider = "oidc"
signin = "/auth/login"
signout = "/auth/logout"
session-cookie = "session"

[output.htmx.authz]
default-access = "public"
default-fallback = "/access-denied"
```

## Step 2: Mark Protected Pages

Add frontmatter to pages requiring auth:

```yaml
---
authn: authenticated
authz:
  - staff
  - admin
---

# Internal Roadmap

This content is only for staff...
```

## Step 3: Implement Auth Middleware

```python
from functools import wraps
from flask import redirect, session

def require_auth(page):
    def decorator(f):
        @wraps(f)
        def decorated(*args, **kwargs):
            if page['authn'] == 'authenticated':
                if 'user' not in session:
                    return redirect('/auth/login')

            if page['authz']:
                user_roles = session.get('user', {}).get('roles', [])
                if not any(r in user_roles for r in page['authz']):
                    return redirect(page.get('fallback', '/access-denied'))

            return f(*args, **kwargs)
        return decorated
    return decorator
```

## Step 4: Create Access Denied Page

Create `src/access-denied.md`:

```markdown
---
title: Access Denied
authn: public
---

# Access Denied

You don't have permission to view this page.

[Request Access](/request-access)
```

## Verification

- [ ] Protected pages redirect to login
- [ ] Unauthorized users see fallback
- [ ] Authorized users can access

## Next Steps

- [Production Deployment](production-deploy.md)
