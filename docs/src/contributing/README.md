---
title: Contributing
description: How to contribute to mdbook-htmx
---

# Contributing

Thank you for your interest in contributing to mdbook-htmx! This guide will help you get started.

## Ways to Contribute

- **Bug Reports** - Found a bug? Open an issue with details
- **Feature Requests** - Have an idea? Discuss it in an issue first
- **Documentation** - Improve docs, fix typos, add examples
- **Code** - Fix bugs, implement features, improve tests

## Section Contents

| Page | Description |
|------|-------------|
| [Development Setup](development.md) | Local development environment |
| [Architecture Guide](architecture.md) | Code structure and modules |
| [Releasing](releasing.md) | Version and publish process |

## Quick Start

```bash
# Clone the repository
git clone https://github.com/aRustyDev/mdbook-htmx
cd mdbook-htmx

# Build and test
cargo build
cargo test

# Run with example book
cargo run -- build example-book/
```

## Guidelines

### Before You Start

1. **Check existing issues** - Someone may already be working on it
2. **Discuss first** - For significant changes, open an issue
3. **Small PRs** - Easier to review and merge

### Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Add tests for new functionality
- Update documentation for user-facing changes

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(search): add Meilisearch integration
fix(render): handle empty frontmatter
docs(readme): update installation instructions
```

### Pull Requests

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes with tests
4. Submit PR with clear description
5. Address review feedback

## Getting Help

- Open an issue for questions
- Check existing issues and discussions
- Review the [Architecture Guide](architecture.md)

## Code of Conduct

Be respectful, constructive, and inclusive. We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
