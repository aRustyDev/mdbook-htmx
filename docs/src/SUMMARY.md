# Summary

[Introduction](README.md)

---

# Concepts

- [Overview](concepts/README.md)
  - [Why HTMX?](concepts/why-htmx.md)
  - [Architecture](concepts/architecture.md)
  - [Build Output](concepts/build-output.md)
  - [Server Model](concepts/server-model.md)

---

# Getting Started

- [Overview](getting-started/README.md)
  - [Installation](getting-started/installation.md)
  - [Quick Start](getting-started/quick-start.md)
  - [Project Structure](getting-started/project-structure.md)

---

# Configuration

- [Overview](configuration/README.md)
  - [book.toml Reference](configuration/book-toml.md)
  - [Frontmatter](configuration/frontmatter.md)
  - [Navigation](configuration/navigation.md)
  - [Theming](configuration/theming.md)

---

# Features

- [Overview](features/README.md)
  - [HTMX Navigation](features/htmx-navigation.md)
  - [Search](features/search.md)
  - [Authentication](features/authentication.md)
  - [Authorization](features/authorization.md)

---

# Deployment

- [Overview](deployment/README.md)
- [Static Hosting](deployment/static/README.md)
  - [GitHub Pages](deployment/static/github-pages.md)
  - [Cloudflare Pages](deployment/static/cloudflare-pages.md)
- [Docker](deployment/docker/README.md)
  - [Basic Setup](deployment/docker/basic.md)
  - [With Search](deployment/docker/with-search.md)
- [Cloudflare Workers](deployment/cloudflare/README.md)
  - [Basic Workers](deployment/cloudflare/workers.md)
  - [Workers + D1](deployment/cloudflare/workers-d1.md)
  - [Workers + Meilisearch](deployment/cloudflare/workers-meilisearch.md)
- [Kubernetes](deployment/kubernetes/README.md)
  - [Basic Deployment](deployment/kubernetes/basic.md)
  - [Helm Chart](deployment/kubernetes/helm.md)
  - [Ingress Authentication](deployment/kubernetes/ingress-auth.md)
  - [With Meilisearch](deployment/kubernetes/with-meilisearch.md)

---

# Advanced Topics

- [Overview](advanced/README.md)
  - [Server Integration](advanced/server-integration.md)
  - [Micro-Frontends](advanced/micro-frontends.md)
  - [CI/CD Pipelines](advanced/cicd.md)
  - [Visual Testing](advanced/visual-testing.md)

---

# Tutorials

- [Overview](tutorials/README.md)
  - [Your First Docs Site](tutorials/first-docs-site.md)
  - [Adding Search](tutorials/adding-search.md)
  - [Adding Authentication](tutorials/adding-auth.md)
  - [Production Deployment](tutorials/production-deploy.md)

---

# Reference

- [Overview](reference/README.md)
  - [Configuration Schema](reference/config-schema.md)
  - [Frontmatter Schema](reference/frontmatter-schema.md)
  - [Manifest Schema](reference/manifest-schema.md)
  - [Search Index Schema](reference/search-index-schema.md)
  - [Template API](reference/template-api.md)

---

# Contributing

- [Overview](contributing/README.md)
  - [Development Setup](contributing/development.md)
  - [Architecture Guide](contributing/architecture.md)
  - [Releasing](contributing/releasing.md)

---

# Architecture Decision Records

- [Overview](adr/README.md)
  - [ADR-0001: Unified Server Model](adr/0001-unified-server.md)
  - [ADR-0002: Canonical book.json](adr/0002-canonical-book-json.md)
  - [ADR-0003: Fragments vs Pages](adr/0003-fragments-vs-pages.md)
  - [ADR-0004: Search Index Design](adr/0004-search-index-design.md)
  - [ADR-0005: HTMX Attribute Injection](adr/0005-htmx-attribute-injection.md)
  - [ADR-0006: Out-of-Band Updates](adr/0006-oob-updates.md)
  - [ADR-0007: Auth Metadata Model](adr/0007-auth-metadata-model.md)
