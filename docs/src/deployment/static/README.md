---
title: Static Hosting
description: Deploy mdbook-htmx to static hosting platforms
---

# Static Hosting

Static hosting is the simplest deployment option. Your documentation is pre-built HTML served from a CDN.

## When to Use Static Hosting

**Advantages:**
- No server to maintain
- Free or very cheap
- Global CDN distribution
- Excellent performance

**Limitations:**
- No server-side authentication
- No server-side search filtering
- No dynamic content

## Platforms

| Platform | Description |
|----------|-------------|
| [GitHub Pages](github-pages.md) | Free hosting from GitHub repositories |
| [Cloudflare Pages](cloudflare-pages.md) | Free hosting with Cloudflare CDN |

## How It Works

```
Build Time                      Runtime
┌─────────────┐                ┌─────────────┐
│  mdbook     │                │   Browser   │
│   build     │                │             │
└──────┬──────┘                └──────┬──────┘
       │                              │
       ▼                              │
┌─────────────┐     Deploy      ┌─────────────┐
│  Static     │ ────────────▶   │    CDN      │
│  Files      │                 │             │
└─────────────┘                 └──────┬──────┘
                                       │
                                       ▼
                                ┌─────────────┐
                                │  Static     │
                                │  Response   │
                                └─────────────┘
```

## HTMX Still Works

Even with static hosting, HTMX navigation works:

1. Initial request returns full page from `pages/`
2. HTMX clicks fetch fragments from `fragments/`
3. Content swaps happen client-side

The only limitation is you can't filter content server-side.

## Search Options

With static hosting, search is client-side:

```javascript
// Load the search index
const index = await fetch('/search-index.json').then(r => r.json());

// Filter locally
function search(query) {
  return index.documents.filter(doc =>
    doc.title.toLowerCase().includes(query.toLowerCase())
  );
}
```

## Next Steps

- [GitHub Pages](github-pages.md) - Deploy with GitHub Actions
- [Cloudflare Pages](cloudflare-pages.md) - Deploy to Cloudflare
