---
title: Workers + Meilisearch
description: Secure Meilisearch via Cloudflare Tunnel
---

# Workers + Meilisearch

Securely access self-hosted Meilisearch via Cloudflare Tunnel.

## Architecture

```
Browser → Worker (Edge) → Cloudflare Tunnel → Meilisearch (Origin)
```

## Setup Cloudflare Tunnel

1. Install cloudflared on your Meilisearch server
2. Create tunnel:

```bash
cloudflared tunnel create meili-tunnel
cloudflared tunnel route dns meili-tunnel meili.internal.example.com
```

3. Configure tunnel:

```yaml
# config.yml
tunnel: your-tunnel-id
credentials-file: /path/to/credentials.json

ingress:
  - hostname: meili.internal.example.com
    service: http://localhost:7700
  - service: http_status:404
```

## Worker Configuration

```toml
# wrangler.toml
[vars]
MEILISEARCH_URL = "https://meili.internal.example.com"

[[secrets]]
name = "MEILISEARCH_KEY"
```

## Search Endpoint

```typescript
export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);

    if (url.pathname === '/search') {
      const query = url.searchParams.get('q');

      const response = await fetch(`${env.MEILISEARCH_URL}/indexes/docs/search`, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${env.MEILISEARCH_KEY}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ q: query }),
      });

      const results = await response.json();
      return renderSearchResults(results);
    }

    // ... rest of handler
  },
};
```

## Zero Trust Integration

For additional security, use Cloudflare Access:

1. Create Access application for the tunnel
2. Add service token authentication
3. Include token in Worker requests

## See Also

- [Search Feature](../../features/search.md)
- [Kubernetes with Meilisearch](../kubernetes/with-meilisearch.md)
