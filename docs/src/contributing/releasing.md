---
title: Releasing
description: Version and publish process
---

# Releasing

How to release new versions of mdbook-htmx.

## Version Strategy

Follows [Semantic Versioning](https://semver.org/):

- **Major** (1.0.0): Breaking changes
- **Minor** (0.1.0): New features, backward compatible
- **Patch** (0.0.1): Bug fixes

## Release Checklist

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md**
3. **Run tests**: `cargo test`
4. **Create tag**: `git tag v0.1.0`
5. **Push**: `git push origin main --tags`
6. **Publish**: `cargo publish`

## Changelog Format

```markdown
## [0.2.0] - 2024-01-15

### Added
- New feature description

### Changed
- Changed behavior

### Fixed
- Bug fix description

### Removed
- Removed feature
```

## GitHub Release

Create release from tag:

1. Go to Releases
2. Click "Draft new release"
3. Select tag
4. Copy changelog section
5. Publish

## Crates.io

```bash
# Dry run
cargo publish --dry-run

# Publish
cargo publish
```

## Docker Image

The CI automatically builds and pushes Docker images on release tags.

## See Also

- [Contributing Guide](README.md)
- [Development Setup](development.md)
