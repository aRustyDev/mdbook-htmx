# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial project structure
- Core library with module stubs:
  - `config` - Configuration loading and validation
  - `render` - Markdown to HTML rendering
  - `frontmatter` - YAML frontmatter parsing
  - `manifest` - Manifest.json generation
  - `search` - Search index generation
  - `assets` - Asset hashing with SRI
  - `templates` - Tera template engine setup
- Error types with exit codes per ADR-0017
- CI/CD workflows for GitHub Actions
- Dual license (MIT OR Apache-2.0)

## [0.1.0] - Unreleased

### Planned

- Parse RenderContext from stdin
- Load and validate [output.htmx] configuration
- Set up Tera template engine with embedded templates
- Parse frontmatter from chapter files
- Render pages and fragments
- Generate manifest.json

[Unreleased]: https://github.com/aRustyDev/mdbook-htmx/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/aRustyDev/mdbook-htmx/releases/tag/v0.1.0
