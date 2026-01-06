---
title: Concepts
description: Mental models and architecture for understanding mdbook-htmx
---

# Concepts

Before diving into configuration and usage, it helps to understand the mental models behind mdbook-htmx. This section explains the "why" behind design decisions.

## Core Ideas

### HTML Over the Wire

mdbook-htmx follows the [HTMX philosophy](https://htmx.org/essays/): instead of returning JSON data and rendering it client-side with JavaScript, the server returns HTML fragments that are swapped directly into the DOM.

This means:
- No JavaScript framework required
- Server controls the rendering
- Progressive enhancement works naturally
- Simpler debugging (view source works)

Read more: [Why HTMX?](why-htmx.md)

### Canonical Data Model

The build process produces `book.json` as the canonical representation of your documentation. HTML files are derived artifacts that can be regenerated. This separation enables:

- Server-side template customization
- Multiple output formats from one source
- Clean integration with dynamic applications

Read more: [Build Output](build-output.md)

### Unified Server Model

Rather than splitting functionality across multiple binaries, mdbook-htmx is designed around a unified server model where one process handles:

- Static file serving
- HTMX fragment responses
- REST API endpoints
- Search queries

Read more: [Server Model](server-model.md)

## Section Contents

| Page | Description |
|------|-------------|
| [Why HTMX?](why-htmx.md) | The philosophy behind HTMX and why it fits documentation |
| [Architecture](architecture.md) | System overview and data flow |
| [Build Output](build-output.md) | What the build produces and why |
| [Server Model](server-model.md) | Unified server design and integration patterns |

## Next Steps

Once you understand the concepts, proceed to [Getting Started](../getting-started/README.md) to install and configure mdbook-htmx.
