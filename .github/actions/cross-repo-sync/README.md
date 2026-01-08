# Cross-Repo Sync

A GitHub Action that synchronizes content between repositories with cryptographic attestation support. Uses tokens from the xauth (cross-repo-auth) GitHub App.

## Features

- Git worktree-based isolated sync operations
- Draft PR workflow with automatic finalization
- Three-layer co-author attribution (xauth + sync-bot + actor)
- Cryptographic attestation via GitHub Artifact Attestations
- PR template discovery and usage
- Comprehensive cleanup on success or failure

## Usage

### Basic Usage

```yaml
- name: Get cross-repo token
  id: xauth
  uses: arustydev/gha/cross-repo-auth@v1
  with:
    app-id: ${{ secrets.XAUTH_APP_ID }}
    private-key: ${{ secrets.XAUTH_PRIVATE_KEY }}
    target-repos: arustydev/docs

- name: Sync docs
  uses: ./.github/actions/cross-repo-sync
  with:
    token: ${{ steps.xauth.outputs.token }}
    sync-type: docs
    source-convention: 'docs/*/book.toml'
    target-repo: arustydev/docs
    target-path: 'library/gh/${{ github.event.repository.name }}'
    xauth-app-id: ${{ steps.xauth.outputs.app-id }}
    xauth-installation-id: ${{ steps.xauth.outputs.installation-id }}
    xauth-issued-at: ${{ steps.xauth.outputs.issued-at }}
```

### With Custom Identity

```yaml
- uses: ./.github/actions/cross-repo-sync
  with:
    token: ${{ steps.xauth.outputs.token }}
    sync-type: docs
    source-convention: 'docs/**/*.md'
    target-repo: arustydev/docs
    target-path: 'library/gh/${{ github.event.repository.name }}'
    xauth-app-id: ${{ steps.xauth.outputs.app-id }}
    xauth-installation-id: ${{ steps.xauth.outputs.installation-id }}
    xauth-issued-at: ${{ steps.xauth.outputs.issued-at }}
    sync-bot-user: 'documentation-bot'
    sync-bot-email: 'docs@mycompany.com'
```

### Disable Attestation

```yaml
- uses: ./.github/actions/cross-repo-sync
  with:
    # ... required inputs ...
    enable-attestation: 'false'
```

## Inputs

### Required

| Input | Description |
|-------|-------------|
| `token` | GitHub token from xauth App |
| `sync-type` | Type of sync: `docs`, `notes`, `blog`, `config` |
| `source-convention` | Glob pattern for source files |
| `target-repo` | Target repository (`owner/repo`) |
| `target-path` | Destination path in target repo |
| `xauth-app-id` | GitHub App ID from xauth |
| `xauth-installation-id` | Installation ID from xauth |
| `xauth-issued-at` | Token issuance timestamp from xauth |

### Optional

| Input | Default | Description |
|-------|---------|-------------|
| `git-user` | `cross-repo-sync[bot]` | Git user name for commits |
| `git-email` | `cross-repo-sync[bot]@users.noreply.github.com` | Git email for commits |
| `sync-bot-user` | `<sync-type>-sync[bot]` | Sync-specific bot name |
| `sync-bot-email` | `<sync-type>-sync@arusty.dev` | Sync-specific bot email |
| `email-domain` | `arusty.dev` | Email domain for sync bot |
| `pr-title` | Auto-generated | PR title override |
| `pr-labels` | (none) | Additional PR labels (comma-separated) |
| `enable-attestation` | `true` | Create attestation for sync |

## Outputs

| Output | Description |
|--------|-------------|
| `pr-number` | Created PR number |
| `pr-url` | URL to the created PR |
| `branch` | Feature branch name |
| `commit-sha` | Commit SHA in target repo |
| `attestation-id` | Attestation ID (or `UNATTESTED`) |
| `manifest-digest` | SHA256 digest of sync-manifest.json |

## Required Permissions

```yaml
permissions:
  id-token: write      # OIDC token for Sigstore signing
  attestations: write  # Create attestations
  contents: read       # Read source files
```

## Attestation

This action creates cryptographic attestations using GitHub Artifact Attestations.

### Verification

**Important**: Attestations are stored in the **source** repository (where the workflow ran), not the target repository.

```bash
# Verify against source repo
gh attestation verify sync-manifest.json \
  --repo owner/source-repo \
  --predicate-type 'https://arusty.dev/attestation/cross-repo-sync/v1'
```

### Manifest Schema

The `sync-manifest.json` file contains:

```json
{
  "version": "1.0",
  "sync_id": "12345678-1",
  "timestamp": "2024-01-15T10:30:00Z",
  "sync": {
    "type": "docs",
    "source_path": "docs/*/book.toml",
    "target_path": "library/gh/mdbook-htmx",
    "files_synced": 42
  },
  "source": {
    "repository": "arustydev/mdbook-htmx",
    "ref": "refs/heads/main",
    "sha": "abc123..."
  },
  "target": {
    "repository": "arustydev/docs",
    "branch": "docs/mdbook-htmx/abc123d",
    "pr_number": 42,
    "commits": ["def789..."]
  }
}
```

## Workflow

1. **Setup**: Clone target repo, create worktree with feature branch
2. **Sync**: Copy files, commit with co-authors, push
3. **Attestation**: Create manifest, generate attestation
4. **Finalize**: Update PR body, mark ready for review, cleanup

## Co-Author Attribution

Commits include three co-authors:

```
Co-Authored-By: cross-repo-auth[bot] <123456+xauth-bot@users.noreply.github.com>
Co-Authored-By: docs-sync[bot] <docs-sync@arusty.dev>
Co-Authored-By: john-doe <john-doe@users.noreply.github.com>
```

| Layer | Purpose |
|-------|---------|
| xauth App | Proves authentication source |
| Sync Bot | Identifies sync purpose |
| Actor | Credits human trigger |

## Error Handling

| Scenario | Behavior |
|----------|----------|
| No files match pattern | Warning logged, action succeeds |
| Attestation fails | Warning logged, PR marked as unattested |
| Push fails | Action fails, cleanup runs |
| PR creation fails | Action fails, cleanup runs |

## Testing

### Unit Tests

Run the unit test workflow to verify action logic:

```bash
gh workflow run test-cross-repo-sync.yml
```

### E2E Tests

Run the E2E test workflow to perform an actual cross-repo sync:

```bash
# Requires CROSS_REPO_PAT secret with repo scope
gh workflow run e2e-cross-repo-sync.yml \
  -f target-repo=arustydev/docs \
  -f target-path=library/gh/mdbook-htmx \
  -f cleanup=true
```

**Prerequisites**:
1. Configure `CROSS_REPO_PAT` secret with a PAT that has `repo` scope
2. PAT must have write access to the target repository

The E2E test:
- Syncs docs to the target repo
- Creates a PR with attestation
- Verifies PR creation and attestation
- Cleans up (closes PR, deletes branch)

## Related

- [ADR-0001: Worktree Strategy](../../.claude/plans/cross-repo-sync-action/adr/0001-worktree-strategy.md)
- [ADR-0002: Draft PR Workflow](../../.claude/plans/cross-repo-sync-action/adr/0002-draft-pr-workflow.md)
- [ADR-0003: Co-Author Attribution](../../.claude/plans/cross-repo-sync-action/adr/0003-co-author-attribution.md)
- [ADR-0004: Attestation Strategy](../../.claude/plans/cross-repo-sync-action/adr/0004-attestation-strategy.md)
