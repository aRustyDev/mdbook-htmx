---
title: Features
description: Core features of mdbook-htmx
---

# Features

mdbook-htmx provides several key features that enhance your documentation:

## Core Features

### HTMX Navigation

Fast, SPA-like navigation without JavaScript frameworks. Links are enhanced with HTMX attributes for instant page transitions.

```html
<a href="/chapter-2"
   hx-get="/fragments/chapter-2.html"
   hx-target="#content"
   hx-push-url="true">Chapter 2</a>
```

### Full-Text Search

Client-side or server-side search with scope filtering:

- **Client-side**: Load `search-index.json` and filter in JavaScript
- **Server-side**: Meilisearch integration with auth-aware filtering

### Authentication & Authorization

Built-in support for access control:

- **Authentication**: Mark pages as requiring login
- **Authorization**: Role-based access control
- **Scoping**: Audience-based content filtering

## Section Contents

| Page | Description |
|------|-------------|
| [HTMX Navigation](htmx-navigation.md) | Fragments, OOB updates, hx-boost |
| [Search](search.md) | Client-side index and Meilisearch |
| [Authentication](authentication.md) | Login requirements and providers |
| [Authorization](authorization.md) | Roles, scopes, and fallbacks |

## Feature Matrix

| Feature | Static Hosting | Server Required |
|---------|----------------|-----------------|
| HTMX Navigation | Yes | Yes |
| Client-side Search | Yes | Yes |
| Server-side Search | No | Yes |
| Authentication | No | Yes |
| Authorization | No | Yes |

## Next Steps

- [HTMX Navigation](htmx-navigation.md) for understanding navigation patterns
- [Search](search.md) for search configuration
