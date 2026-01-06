---
title: Installation
description: Install mdbook-htmx from crates.io or source
---

# Installation

Install mdbook-htmx to start creating HTMX-enhanced documentation.

## From crates.io (Recommended)

The easiest way to install:

```bash
cargo install mdbook-htmx
```

This installs the latest stable release.

## From Source

For the latest development version:

```bash
git clone https://github.com/aRustyDev/mdbook-htmx
cd mdbook-htmx
cargo install --path .
```

## Verify Installation

Check that the installation succeeded:

```bash
mdbook-htmx --version
```

## Prerequisites

mdbook-htmx requires:

- **Rust 1.70+** - Install via [rustup](https://rustup.rs/)
- **mdBook** - Optional but recommended for the `mdbook` CLI

## Next Steps

- [Quick Start](quick-start.md) - Create your first docs site
- [Project Structure](project-structure.md) - Understand the file layout
