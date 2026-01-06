---
title: Navigation
description: Configure navigation components
---

# Navigation

Configure breadcrumbs, table of contents, and prev/next navigation.

## Configuration

```toml
[output.htmx.navigation]
breadcrumbs = true
toc = true
prev-next = true
collapsible-sidebar = true
```

## Breadcrumbs

Breadcrumbs show the path to the current page:

```
Home > Getting Started > Installation
```

Enable in book.toml:

```toml
[output.htmx.navigation]
breadcrumbs = true
```

## Table of Contents

The TOC shows headings within the current page:

```toml
[output.htmx.navigation]
toc = true
```

TOC depth follows `heading-split-level` from search config.

## Previous/Next

Links to adjacent pages in the navigation order:

```toml
[output.htmx.navigation]
prev-next = true
```

## Collapsible Sidebar

Sections can collapse in the sidebar:

```toml
[output.htmx.navigation]
collapsible-sidebar = true
```

## HTMX Updates

Navigation components update via OOB (Out-of-Band) swaps when navigating with HTMX. See [OOB Updates](../adr/0006-oob-updates.md) for details.

## See Also

- [HTMX Navigation](../features/htmx-navigation.md) - How navigation works
- [Theming](theming.md) - Customize appearance
