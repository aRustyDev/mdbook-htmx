---
title: Advanced Topics
description: Advanced patterns and integrations for mdbook-htmx
---

# Advanced Topics

This section covers advanced usage patterns for power users and platform engineers.

## Section Contents

| Page | Description |
|------|-------------|
| [Server Integration](server-integration.md) | REST API and GraphQL patterns |
| [Micro-Frontends](micro-frontends.md) | Embedding fragments in applications |
| [CI/CD Pipelines](cicd.md) | Automated builds and deployments |
| [Visual Testing](visual-testing.md) | Playwright and screenshot testing |

## Overview

### Server Integration

Integrate mdbook-htmx with your existing backend:

- Load the manifest for routing decisions
- Serve fragments through your API layer
- Combine with existing authentication

### Micro-Frontends

Use documentation fragments as part of a larger application:

- Embed docs in admin panels
- Load help content contextually
- Share navigation with main app

### CI/CD Pipelines

Automate your documentation workflow:

- Build on push to main
- Update search indexes
- Deploy to multiple environments

### Visual Testing

Ensure documentation renders correctly:

- Screenshot comparison tests
- Cross-browser testing
- Accessibility validation

## Prerequisites

These topics assume familiarity with:

- [Concepts](../concepts/README.md) - Architecture and mental models
- [Configuration](../configuration/README.md) - book.toml and frontmatter
- [Deployment](../deployment/README.md) - Basic deployment patterns
