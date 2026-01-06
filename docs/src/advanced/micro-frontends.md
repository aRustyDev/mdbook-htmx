---
title: Micro-Frontends
description: Embed documentation fragments in applications
---

# Micro-Frontends

Use documentation fragments as components in larger applications.

## The Pattern

Load documentation content into your app without full page loads:

```html
<div id="docs-panel">
  <button hx-get="/docs/fragments/getting-started.html"
          hx-target="#docs-content"
          hx-swap="innerHTML">
    Getting Started
  </button>
  <div id="docs-content"></div>
</div>
```

## Shared Navigation

Keep navigation in sync with your main app:

```javascript
// Listen for HTMX navigation events
document.body.addEventListener('htmx:afterSwap', (e) => {
  if (e.detail.target.id === 'docs-content') {
    updateAppNavigation(e.detail.pathInfo.requestPath);
  }
});
```

## Style Integration

Match documentation styles to your app:

```css
#docs-panel {
  /* Override docs styles */
  --primary-color: var(--app-primary);
  --background: var(--app-background);
}
```

## Contextual Help

Load help content based on context:

```javascript
function showHelp(topic) {
  htmx.ajax('GET', `/docs/fragments/help/${topic}.html`, '#help-modal');
  document.getElementById('help-modal').showModal();
}
```

## See Also

- [HTMX Navigation](../features/htmx-navigation.md)
- [Server Integration](server-integration.md)
