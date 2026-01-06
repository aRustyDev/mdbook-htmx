---
title: "ADR-0006: Out-of-Band Updates"
description: Using HTMX OOB swaps for navigation components
---

# ADR-0006: Out-of-Band Updates

## Status

Accepted

## Context

When navigating with HTMX, only the main content swaps. But other elements need updating too:

- Breadcrumbs
- Table of contents
- Previous/next links
- Active navigation state

Options:
1. **JavaScript handlers**: Update elements via htmx:afterSwap
2. **Multiple requests**: Fetch each component separately
3. **OOB swaps**: Include updates in the same response

## Decision

Use **HTMX Out-of-Band (OOB) swaps** to update multiple elements in a single response.

### Fragment Structure

```html
<!-- Main content (swapped into #content) -->
<article id="content">
  <h1>Chapter 2</h1>
  ...
</article>

<!-- OOB updates (swapped by ID) -->
<nav id="breadcrumbs" hx-swap-oob="true">
  Home > Guide > Chapter 2
</nav>

<nav id="toc" hx-swap-oob="true">
  <a href="#section-1">Section 1</a>
  ...
</nav>

<nav id="prev-next" hx-swap-oob="true">
  <a href="/chapter-1">← Previous</a>
  <a href="/chapter-3">Next →</a>
</nav>
```

## Consequences

### Positive

- **Single request**: All updates in one response
- **Server authority**: Server controls what updates
- **Declarative**: No JavaScript needed
- **Atomic**: All elements update together
- **Simple templates**: Each component is self-contained

### Negative

- **Larger responses**: More HTML per request
- **ID coupling**: Elements must have stable IDs
- **Debugging**: OOB swaps can be confusing to debug

### Mitigations

- Only include OOB elements that actually changed
- Use consistent ID naming scheme
- HTMX extension for debugging: htmx:oobAfterSwap event

## Related

- [ADR-0003](0003-fragments-vs-pages.md) - Fragment design
- [Navigation](../configuration/navigation.md) - Navigation config
