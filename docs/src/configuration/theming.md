---
title: Theming
description: Customize appearance with CSS variables and templates
---

# Theming

Customize the look and feel of your documentation.

## CSS Variables

Override CSS variables in a custom stylesheet:

```css
:root {
  --primary-color: #3b82f6;
  --secondary-color: #64748b;
  --background: #ffffff;
  --foreground: #1e293b;
  --sidebar-width: 280px;
  --content-max-width: 800px;
}
```

## Custom Theme Directory

Point to a custom theme directory:

```toml
[output.htmx]
theme-dir = "theme"
```

Then create custom templates:

```
theme/
├── layout.html      # Base layout
├── page.html        # Full page template
├── fragment.html    # Fragment template
└── styles.css       # Custom styles
```

## Template Variables

Templates receive these context variables:

| Variable | Description |
|----------|-------------|
| `page` | Current page data |
| `book` | Full book structure |
| `config` | HTMX configuration |
| `navigation` | Nav structure |

## Dark Mode

The default theme supports dark mode via media query:

```css
@media (prefers-color-scheme: dark) {
  :root {
    --background: #0f172a;
    --foreground: #f8fafc;
  }
}
```

## See Also

- [Template API](../reference/template-api.md) - Complete context reference
- [Navigation](navigation.md) - Navigation components
