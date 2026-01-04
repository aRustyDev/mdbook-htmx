# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- HTMX attribute injection module (`render/htmx.rs`)
  - Injects `hx-boost`, `hx-target`, `hx-swap` on `<body>`
  - Generates navigation link attributes
  - Preload hints for hover prefetching
- Out-of-Band (OOB) swap generation (`render/oob.rs`)
  - Sidebar OOB updates with active state
  - Breadcrumb OOB updates
  - NavItem and Breadcrumb data structures
- New templates for HTMX integration
  - `nav.html` - Main navigation component with HTMX attributes
  - `partials/sidebar-oob.html` - Sidebar for OOB swaps
  - `partials/breadcrumb.html` - Breadcrumb navigation trail
  - `partials/loading.html` - HTMX loading indicator
- Updated layout template with:
  - Sidebar navigation panel
  - Loading spinner indicator
  - Breadcrumb placeholder
  - HTMX event handlers for enhanced UX
  - Mobile responsive design
- Fragment template now includes:
  - OOB update placeholder
  - Lazy loading support

### Changed

- Updated `render/mod.rs` to export HTMX and OOB modules
- Enhanced `HtmxRenderer.render_chapter()` to:
  - Build navigation sidebar context
  - Generate OOB updates for fragments
  - Pass all chapters for navigation building

## [0.1.0] - 2024-01-04

### Added

- Core library modules:
  - `config` - Configuration loading from `[output.htmx]`
  - `context` - RenderContext parsing from stdin JSON
  - `render` - Markdown to HTML with pulldown-cmark
  - `frontmatter` - YAML frontmatter parsing and validation
  - `manifest` - manifest.json generation
  - `search` - Search index generation (stub)
  - `assets` - Asset hashing with SRI (SHA-384) and xxHash
  - `templates` - Tera template engine with custom filters
- HtmxRenderer with full render pipeline
- Embedded templates (layout.html, docs/page.html, docs/fragment.html)
- Full HtmxConfig with navigation, search, and assets options
- Prev/next chapter navigation
- GitHub-compatible heading slugification
- Error types with exit codes per ADR-0017
- CI/CD workflows for GitHub Actions
- Dual license (MIT OR Apache-2.0)

[Unreleased]: https://github.com/aRustyDev/mdbook-htmx/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/aRustyDev/mdbook-htmx/releases/tag/v0.1.0
