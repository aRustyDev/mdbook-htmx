---
title: Architecture Decision Records
description: Key architectural decisions in mdbook-htmx
---

# Architecture Decision Records

Architecture Decision Records (ADRs) capture important design decisions and their rationale. They help future contributors understand why things work the way they do.

## What's an ADR?

An ADR documents:

- **Context** - What situation prompted the decision
- **Decision** - What was decided
- **Consequences** - What are the trade-offs
- **Status** - Accepted, superseded, deprecated

## ADR Index

| ADR | Title | Status |
|-----|-------|--------|
| [0001](0001-unified-server.md) | Unified Server Model | Accepted |
| [0002](0002-canonical-book-json.md) | Canonical book.json | Accepted |
| [0003](0003-fragments-vs-pages.md) | Fragments vs Pages | Accepted |
| [0004](0004-search-index-design.md) | Search Index Design | Accepted |
| [0005](0005-htmx-attribute-injection.md) | HTMX Attribute Injection | Accepted |
| [0006](0006-oob-updates.md) | Out-of-Band Updates | Accepted |
| [0007](0007-auth-metadata-model.md) | Auth Metadata Model | Accepted |

## Reading ADRs

Each ADR follows this structure:

```markdown
# ADR-NNNN: Title

## Status
Accepted | Superseded by ADR-XXXX | Deprecated

## Context
What is the issue that we're seeing that is motivating this decision?

## Decision
What is the change that we're proposing and/or doing?

## Consequences
What becomes easier or more difficult because of this change?
```

## When to Write an ADR

Write an ADR when:

- Making a significant architectural decision
- Choosing between multiple viable approaches
- The decision affects multiple components
- Future maintainers might question "why"

## Proposing New ADRs

To propose a new architectural decision:

1. Open an issue describing the decision needed
2. Discuss alternatives in the issue
3. Write the ADR document
4. Submit as part of the implementing PR

## Related

- [Concepts: Architecture](../concepts/architecture.md) - System overview
- [Contributing: Architecture Guide](../contributing/architecture.md) - Code structure
