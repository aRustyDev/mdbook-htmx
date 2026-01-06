---
title: HTMX Navigation
description: SPA-like navigation with HTMX
---

# HTMX Navigation

mdbook-htmx enhances navigation links with HTMX attributes for instant page transitions.

## How It Works

Standard links are enhanced:

```html
<!-- Before enhancement -->
<a href="/chapter-2">Chapter 2</a>

<!-- After enhancement -->
<a href="/chapter-2"
   hx-get="/fragments/chapter-2.html"
   hx-target="#content"
   hx-swap="innerHTML"
   hx-push-url="true">Chapter 2</a>
```

## hx-boost

When `boost = true` (default), the body gets `hx-boost="true"`:

```html
<body hx-boost="true">
```

This enhances all links and forms automatically.

## Fragments

Fragments contain only the content portion:

```html
<article id="content">
  <h1>Chapter 2</h1>
  <p>Content here...</p>
</article>
```

## OOB Updates

Fragments include Out-of-Band updates for navigation components:

```html
<!-- Main content -->
<article id="content">...</article>

<!-- OOB updates -->
<nav id="breadcrumbs" hx-swap-oob="true">...</nav>
<nav id="toc" hx-swap-oob="true">...</nav>
<nav id="prev-next" hx-swap-oob="true">...</nav>
```

## Progressive Enhancement

Without JavaScript, links work normally. With JavaScript, HTMX provides instant navigation.

## See Also

- [Why HTMX?](../concepts/why-htmx.md) - Philosophy
- [OOB Updates ADR](../adr/0006-oob-updates.md) - Design decision
