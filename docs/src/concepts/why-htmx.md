---
title: Why HTMX?
description: The philosophy behind HTMX and why it's a good fit for documentation sites
---

# Why HTMX?

HTMX represents a return to the web's original architecture: the server returns HTML, and the browser renders it. But unlike traditional multi-page applications, HTMX enables partial page updates without full page reloads.

## The Problem with SPAs

Single Page Applications (SPAs) have become the default for modern web development, but they come with costs:

| Concern | SPA Approach | Traditional MPA |
|---------|-------------|-----------------|
| Initial load | Large JS bundle | Fast HTML |
| Rendering | Client-side | Server-side |
| SEO | Requires SSR/hydration | Works naturally |
| Debugging | React DevTools, etc. | View Source |
| Complexity | Framework + state management | HTML + forms |

For documentation sites, SPAs are often overkill. You're primarily displaying static content with navigation.

## HTML Over the Wire

HTMX takes a different approach:

```html
<!-- Traditional link -->
<a href="/chapter-2">Chapter 2</a>

<!-- HTMX-enhanced link -->
<a href="/chapter-2"
   hx-get="/fragments/chapter-2.html"
   hx-target="#content"
   hx-swap="innerHTML"
   hx-push-url="true">Chapter 2</a>
```

When clicked, HTMX:
1. Makes a GET request to `/fragments/chapter-2.html`
2. Swaps the response into `#content`
3. Updates the browser URL to `/chapter-2`

The result feels like an SPA, but:
- No JavaScript framework required
- Server controls what HTML to return
- Works without JavaScript (progressive enhancement)
- Standard HTTP caching applies

## Why It Fits Documentation

Documentation has specific characteristics that align with HTMX:

### Content is Mostly Static

Markdown doesn't change between requests. The server can return the same HTML fragment repeatedly, and HTTP caching handles performance.

### Navigation is Predictable

Documentation has a clear structure: table of contents, next/previous, breadcrumbs. These can all be HTMX-enhanced without complex client-side state.

### Progressive Enhancement Matters

Users may have JavaScript disabled, slow connections, or accessibility needs. HTMX links work as regular links when JavaScript isn't available.

### Search is the Complex Part

The only truly dynamic feature in most documentation is search. HTMX handles this elegantly:

```html
<input type="search"
       hx-get="/search"
       hx-trigger="keyup changed delay:300ms"
       hx-target="#results">
```

Results are returned as HTML, ready to render.

## Out-of-Band Updates

HTMX supports updating multiple page elements in a single response using Out-of-Band (OOB) swaps:

```html
<!-- Main content -->
<div id="content">
  <h1>Chapter 2</h1>
  ...
</div>

<!-- OOB updates sent in same response -->
<nav id="breadcrumbs" hx-swap-oob="true">
  Home > Guide > Chapter 2
</nav>

<nav id="toc" hx-swap-oob="true">
  <a href="#section-1">Section 1</a>
  ...
</nav>
```

This allows updating:
- Breadcrumb navigation
- Table of contents highlighting
- Previous/next links
- Any other page elements

All in a single HTTP response.

## The Mental Model

Think of HTMX as giving HTML superpowers:

| Without HTMX | With HTMX |
|-------------|-----------|
| Click link → full page reload | Click link → swap content |
| Submit form → full page reload | Submit form → swap response |
| Need JS for dynamic UI | HTML attributes handle it |
| Server returns JSON | Server returns HTML |

For documentation, this means you can have instant-feeling navigation without:
- Bundlers
- Framework dependencies
- Complex build pipelines
- Client-side state management

## Further Reading

- [HTMX Documentation](https://htmx.org/docs/)
- [Hypermedia Systems](https://hypermedia.systems/) - Book by HTMX creators
- [Architecture](architecture.md) - How mdbook-htmx implements these concepts
