---
title: Development Setup
description: Set up local development environment
---

# Development Setup

Set up your local environment for contributing.

## Prerequisites

- Rust 1.70+
- Git
- A code editor

## Clone the Repository

```bash
git clone https://github.com/aRustyDev/mdbook-htmx
cd mdbook-htmx
```

## Build

```bash
cargo build
```

## Run Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

## Run Locally

```bash
# Build and run with example
cargo run -- build example-book/

# Watch mode (requires cargo-watch)
cargo watch -x "run -- build example-book/"
```

## Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Check all
cargo fmt && cargo clippy && cargo test
```

## Project Structure

```
mdbook-htmx/
├── src/
│   ├── lib.rs           # Library entry
│   ├── main.rs          # CLI entry
│   ├── config/          # Configuration
│   ├── render/          # Rendering logic
│   ├── frontmatter/     # Frontmatter parsing
│   └── search/          # Search index
├── templates/           # Tera templates
├── docs/                # Documentation
└── tests/               # Integration tests
```

## IDE Setup

### VS Code

Recommended extensions:
- rust-analyzer
- Even Better TOML
- Error Lens

### Settings

```json
{
  "rust-analyzer.check.command": "clippy"
}
```

## See Also

- [Architecture Guide](architecture.md) - Code structure
- [Contributing Guide](README.md) - Guidelines
