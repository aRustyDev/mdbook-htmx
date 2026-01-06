---
title: Template API
description: Tera template context variables reference
---

# Template API

Reference for Tera template context variables.

## Context Structure

Templates receive a context with these top-level variables:

| Variable | Type | Description |
|----------|------|-------------|
| `page` | object | Current page data |
| `book` | object | Full book structure |
| `config` | object | HTMX configuration |
| `navigation` | object | Navigation structure |

## Page Object

```json
{
  "path": "/getting-started",
  "title": "Getting Started",
  "content": "<p>HTML content...</p>",
  "frontmatter": {
    "title": "Getting Started",
    "description": "...",
    "scope": "public",
    "authn": "public"
  },
  "headings": [
    {"level": 1, "text": "Getting Started", "id": "getting-started"},
    {"level": 2, "text": "Installation", "id": "installation"}
  ],
  "prev": {"path": "/intro", "title": "Introduction"},
  "next": {"path": "/config", "title": "Configuration"}
}
```

## Book Object

```json
{
  "title": "My Documentation",
  "authors": ["Author Name"],
  "description": "...",
  "chapters": [...]
}
```

## Config Object

```json
{
  "htmx_version": "1.9.10",
  "boost": true,
  "swap_strategy": "innerHTML",
  "target": "#content"
}
```

## Navigation Object

```json
{
  "toc": [...],
  "breadcrumbs": [
    {"path": "/", "title": "Home"},
    {"path": "/guide", "title": "Guide"}
  ]
}
```

## Template Examples

### Breadcrumbs

```html
<nav id="breadcrumbs">
  {% for crumb in navigation.breadcrumbs %}
    <a href="{{ crumb.path }}">{{ crumb.title }}</a>
    {% if not loop.last %} &gt; {% endif %}
  {% endfor %}
</nav>
```

### Table of Contents

```html
<nav id="toc">
  {% for heading in page.headings %}
    <a href="#{{ heading.id }}"
       class="toc-{{ heading.level }}">
      {{ heading.text }}
    </a>
  {% endfor %}
</nav>
```

### Prev/Next

```html
<nav id="prev-next">
  {% if page.prev %}
    <a href="{{ page.prev.path }}">← {{ page.prev.title }}</a>
  {% endif %}
  {% if page.next %}
    <a href="{{ page.next.path }}">{{ page.next.title }} →</a>
  {% endif %}
</nav>
```

## See Also

- [Theming](../configuration/theming.md) - Custom templates
- [Tera Documentation](https://keats.github.io/tera/)
