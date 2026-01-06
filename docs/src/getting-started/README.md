---
title: Getting Started
description: Install and configure mdbook-htmx
---

# Getting Started

Get up and running with mdbook-htmx in minutes.

## What You'll Learn

- Installing mdbook-htmx from crates.io or source
- Creating your first HTMX-enhanced documentation site
- Understanding the project structure

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (for installation from crates.io)
- [mdBook](https://rust-lang.github.io/mdBook/guide/installation.html) knowledge (optional but helpful)

## Section Contents

| Page | Description |
|------|-------------|
| [Installation](installation.md) | Install from crates.io or build from source |
| [Quick Start](quick-start.md) | Create your first docs site in 5 minutes |
| [Project Structure](project-structure.md) | Directory layout and file organization |

## Quick Install

```bash
cargo install mdbook-htmx
```

Then add to your `book.toml`:

```toml
[output.htmx]
```

See [Installation](installation.md) for detailed instructions.

## Next Steps

After installation, follow the [Quick Start](quick-start.md) guide to create your first site.
