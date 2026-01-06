---
title: Manifest Schema
description: manifest.json structure reference
---

# Manifest Schema

Reference for the `manifest.json` output file.

## Schema URL

```
https://schemas.arusty.dev/mdbook-htmx/manifest.schema.json
```

## Structure

```json
{
  "$schema": "https://schemas.arusty.dev/mdbook-htmx/manifest.schema.json",
  "version": "1.0.0",
  "generated_at": "2024-01-15T10:30:00Z",
  "config": { ... },
  "pages": { ... },
  "assets": { ... }
}
```

## Root Fields

| Field | Type | Description |
|-------|------|-------------|
| `version` | string | Schema version |
| `generated_at` | string | ISO 8601 timestamp |
| `config` | object | Configuration snapshot |
| `pages` | object | Page metadata |
| `assets` | object | Asset metadata |

## Config Object

```json
{
  "htmx_version": "1.9.10",
  "boost": true,
  "swap_strategy": "innerHTML",
  "target": "#content",
  "push_url": true
}
```

## Page Object

```json
{
  "/getting-started": {
    "title": "Getting Started",
    "source": "getting-started.md",
    "page_path": "pages/getting-started.html",
    "fragment_path": "fragments/getting-started.html",
    "scope": "public",
    "authn": "public",
    "authz": [],
    "fallback": null,
    "content_hash": "sha256:abc123...",
    "last_modified": "2024-01-15T10:30:00Z"
  }
}
```

## Asset Object

```json
{
  "css/style.css": {
    "hash": "sha256:...",
    "sri": "sha256-..."
  }
}
```

## Usage

Load and use the manifest:

```python
import json

manifest = json.load(open('manifest.json'))

# Check if page exists
page = manifest['pages'].get('/getting-started')

# Get auth requirements
if page['authn'] == 'authenticated':
    # Require login
    pass
```

## See Also

- [Build Output](../concepts/build-output.md) - Understanding outputs
- [Server Model](../concepts/server-model.md) - Using the manifest
