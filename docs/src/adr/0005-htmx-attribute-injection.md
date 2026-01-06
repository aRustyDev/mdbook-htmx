---
title: "ADR-0005: HTMX Attribute Injection"
description: How HTMX attributes are added to HTML
---

# ADR-0005: HTMX Attribute Injection

## Status

Accepted

## Context

To enable HTMX navigation, links need attributes like:

```html
<a href="/chapter-2"
   hx-get="/fragments/chapter-2.html"
   hx-target="#content"
   hx-swap="innerHTML"
   hx-push-url="true">
```

We needed to decide how and where to add these.

## Decision

**Post-process HTML** to inject HTMX attributes:

1. Parse rendered HTML
2. Find navigation links
3. Add HTMX attributes based on config
4. Preserve original href for progressive enhancement

### Injection Rules

| Element | Action |
|---------|--------|
| Internal links | Add hx-get, hx-target, hx-swap, hx-push-url |
| External links | No modification |
| Anchor links | No modification |
| Forms | Add hx-post if boost enabled |

## Consequences

### Positive

- **Clean Markdown**: Authors write standard Markdown
- **Configurable**: Attributes follow global config
- **Progressive enhancement**: Original links preserved
- **Template-independent**: Works with any template

### Negative

- **Build overhead**: HTML parsing required
- **Limited control**: Per-link customization harder
- **Complexity**: Additional processing step

### Mitigations

- Efficient HTML parser (lol_html or similar)
- Frontmatter can disable boost per page
- Templates can add custom HTMX attributes

## Example

Before:
```html
<a href="/chapter-2">Chapter 2</a>
```

After:
```html
<a href="/chapter-2"
   hx-get="/fragments/chapter-2.html"
   hx-target="#content"
   hx-swap="innerHTML"
   hx-push-url="true">Chapter 2</a>
```

## Related

- [HTMX Navigation](../features/htmx-navigation.md) - Feature docs
- [Why HTMX](../concepts/why-htmx.md) - Philosophy
