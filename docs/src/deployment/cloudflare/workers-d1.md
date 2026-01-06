---
title: Workers + D1
description: Add session storage with Cloudflare D1
---

# Workers + D1

Add session storage for authentication with D1.

## wrangler.toml

```toml
name = "my-docs"
main = "src/index.ts"
compatibility_date = "2024-01-01"

[site]
bucket = "./book/htmx"

[[d1_databases]]
binding = "DB"
database_name = "docs-sessions"
database_id = "your-database-id"
```

## Create Database

```bash
wrangler d1 create docs-sessions

wrangler d1 execute docs-sessions --command "
CREATE TABLE sessions (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  data TEXT,
  expires_at INTEGER NOT NULL
);
"
```

## Session Management

```typescript
interface Session {
  id: string;
  userId: string;
  data: Record<string, unknown>;
  expiresAt: number;
}

async function getSession(env: Env, sessionId: string): Promise<Session | null> {
  const result = await env.DB.prepare(
    'SELECT * FROM sessions WHERE id = ? AND expires_at > ?'
  ).bind(sessionId, Date.now()).first();

  return result as Session | null;
}
```

## Auth Middleware

```typescript
async function authMiddleware(request: Request, env: Env) {
  const sessionId = getCookie(request, 'session');
  if (!sessionId) return null;

  return await getSession(env, sessionId);
}
```

## See Also

- [Authentication](../../features/authentication.md) - Auth configuration
- [Workers + Meilisearch](workers-meilisearch.md) - Add search
