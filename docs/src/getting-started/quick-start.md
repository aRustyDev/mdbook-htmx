---
title: Quick Start
description: Create your first HTMX-enhanced docs site in 5 minutes
---

# Quick Start

Create an HTMX-enhanced documentation site in 5 minutes.

## Create a New Book

```bash
# Create directory structure
mkdir my-docs && cd my-docs
mkdir -p src

# Create book.toml
cat > book.toml << 'EOF'
[book]
title = "My Documentation"
authors = ["Your Name"]

[output.htmx]
EOF

# Create SUMMARY.md
cat > src/SUMMARY.md << 'EOF'
# Summary

[Introduction](README.md)

# Guide

- [Getting Started](getting-started.md)
- [Configuration](configuration.md)
EOF

# Create initial pages
echo "# My Documentation\n\nWelcome!" > src/README.md
echo "# Getting Started\n\nLet's begin." > src/getting-started.md
echo "# Configuration\n\nConfigure your project." > src/configuration.md
```

## Build the Documentation

```bash
mdbook build
```

Output is in `book/htmx/`:

```
book/htmx/
├── pages/
├── fragments/
├── manifest.json
├── search-index.json
└── assets/
```

## Preview Locally

```bash
# Simple HTTP server
python -m http.server 8000 --directory book/htmx/pages

# Or use any static file server
npx serve book/htmx/pages
```

Open http://localhost:8000 in your browser.

## What's Next?

- [Project Structure](project-structure.md) - Understand the file layout
- [Configuration](../configuration/book-toml.md) - Customize the output
- [Tutorials](../tutorials/first-docs-site.md) - Detailed walkthrough
