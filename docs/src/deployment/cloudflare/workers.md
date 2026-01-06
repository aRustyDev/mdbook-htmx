---
title: Cloudflare Workers
description: Deploy to Cloudflare Workers
---

# Cloudflare Workers

Deploy documentation to Cloudflare's edge network.

## wrangler.toml

```toml
name = "my-docs"
main = "src/index.ts"
compatibility_date = "2024-01-01"

[site]
bucket = "./book/htmx"

[[kv_namespaces]]
binding = "DOCS"
id = "your-kv-namespace-id"
```

## Worker Script

```typescript
export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);
    const path = url.pathname;

    // Check for HTMX request
    const isHtmx = request.headers.get('HX-Request') === 'true';

    // Serve fragment or full page
    const file = isHtmx
      ? `fragments${path}.html`
      : `pages${path}.html`;

    const content = await env.DOCS.get(file);
    if (!content) {
      return new Response('Not found', { status: 404 });
    }

    return new Response(content, {
      headers: { 'Content-Type': 'text/html' },
    });
  },
};
```

## Deploy

```bash
# Install Wrangler
npm install -g wrangler

# Login
wrangler login

# Deploy
wrangler deploy
```

## See Also

- [Workers + D1](workers-d1.md) - Add session storage
- [Workers + Meilisearch](workers-meilisearch.md) - Add search
