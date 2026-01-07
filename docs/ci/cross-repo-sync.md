# Cross-Repo Sync

The `cross-repo-sync` action synchronizes content between repositories with cryptographic attestation support.

## Overview

This action enables automated documentation sync from `mdbook-htmx` to the central `arustydev/docs` repository. It uses the `xauth` (cross-repo-auth) GitHub App for authentication and creates cryptographically signed attestations for audit trails.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        CROSS-REPO SYNC WORKFLOW                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐              │
│  │   SOURCE     │    │    XAUTH     │    │    TARGET    │              │
│  │   (mdbook-   │───▶│  (GitHub     │───▶│   (docs      │              │
│  │    htmx)     │    │    App)      │    │    repo)     │              │
│  └──────────────┘    └──────────────┘    └──────────────┘              │
│        │                                        │                       │
│        │                                        ▼                       │
│        │                                 ┌──────────────┐              │
│        │                                 │   DRAFT PR   │              │
│        │                                 │  (with       │              │
│        └─────────────────────────────────│ attestation) │              │
│                                          └──────────────┘              │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## Workflow Files

| Workflow | Purpose |
|----------|---------|
| `docs-sync.yml` | Syncs docs to `arustydev/docs` on push to main |
| `test-cross-repo-sync.yml` | Tests the action's logic |

## Usage

### Basic Sync

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
    source-convention: 'docs/**/*'
    target-repo: arustydev/docs
    target-path: 'library/gh/${{ github.event.repository.name }}'
    xauth-app-id: ${{ steps.xauth.outputs.app-id }}
    xauth-installation-id: ${{ steps.xauth.outputs.installation-id }}
    xauth-issued-at: ${{ steps.xauth.outputs.issued-at }}
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
| `xauth-issued-at` | Token issuance timestamp |

### Optional

| Input | Default | Description |
|-------|---------|-------------|
| `git-user` | `cross-repo-sync[bot]` | Git user for commits |
| `git-email` | Auto-generated | Git email for commits |
| `sync-bot-user` | `<sync-type>-sync[bot]` | Sync-specific bot name |
| `email-domain` | `arusty.dev` | Email domain for sync bot |
| `pr-title` | Auto-generated | PR title override |
| `pr-labels` | (none) | Additional PR labels |
| `enable-attestation` | `true` | Create attestation |

## Outputs

| Output | Description |
|--------|-------------|
| `pr-number` | Created PR number |
| `pr-url` | URL to the created PR |
| `branch` | Feature branch name |
| `commit-sha` | Commit SHA in target repo |
| `attestation-id` | Attestation ID (or `UNATTESTED`) |
| `manifest-digest` | SHA256 digest of sync-manifest.json |

## Attestation

The action creates cryptographic attestations using GitHub Artifact Attestations and Sigstore.

### What's Attested

| Field | Description |
|-------|-------------|
| Sync metadata | Type, paths, file count |
| Source info | Repository, ref, SHA |
| Target info | Repository, branch, PR number |
| xauth info | App ID, installation ID, token timestamp |

### Verification

**Important**: Attestations are stored in the **source** repository, not the target.

```bash
# Download manifest from PR artifacts
gh run download <run-id> -n sync-manifest

# Verify against source repo
gh attestation verify sync-manifest.json \
  --repo arustydev/mdbook-htmx \
  --predicate-type 'https://arusty.dev/attestation/cross-repo-sync/v1'
```

### Predicate Type

```
https://arusty.dev/attestation/cross-repo-sync/v1
```

## Co-Author Attribution

Commits include three co-authors for audit trails:

```
Co-Authored-By: cross-repo-auth[bot] <123456+xauth-bot@users.noreply.github.com>
Co-Authored-By: docs-sync[bot] <docs-sync@arusty.dev>
Co-Authored-By: username <username@users.noreply.github.com>
```

| Layer | Purpose |
|-------|---------|
| xauth App | Proves authentication source |
| Sync Bot | Identifies sync purpose |
| Actor | Credits human trigger |

## Permissions Required

```yaml
permissions:
  id-token: write      # OIDC for Sigstore signing
  attestations: write  # Create attestations
  contents: read       # Read source files
```

## Setup

### Prerequisites

1. **xauth GitHub App** deployed and configured
2. **Secrets** configured in repository:
   - `XAUTH_APP_ID`
   - `XAUTH_PRIVATE_KEY`
3. **Repository variable** set:
   - `XAUTH_ENABLED=true`

### Target Repository

The target repository (`arustydev/docs`) must:
- Have the xauth app installed
- Grant appropriate permissions to the app

## Troubleshooting

### No Files Matched

If the workflow reports "No files matched pattern":

1. Check the glob pattern in `source-convention`
2. Verify files exist at the specified paths
3. Test the pattern locally:
   ```bash
   shopt -s globstar
   ls docs/**/*.md
   ```

### Attestation Failed

If attestation creation fails:

1. Check workflow permissions include `id-token: write` and `attestations: write`
2. Verify GitHub OIDC is working
3. The PR will still be created but marked as "UNATTESTED"

### Push Failed

If git push fails:

1. Verify xauth token has push permissions to target repo
2. Check the target repo branch protection rules
3. Verify the xauth app installation is active

## Design Documents

- [ADR-0001: Worktree Strategy](/.claude/plans/cross-repo-sync-action/adr/0001-worktree-strategy.md)
- [ADR-0002: Draft PR Workflow](/.claude/plans/cross-repo-sync-action/adr/0002-draft-pr-workflow.md)
- [ADR-0003: Co-Author Attribution](/.claude/plans/cross-repo-sync-action/adr/0003-co-author-attribution.md)
- [ADR-0004: Attestation Strategy](/.claude/plans/cross-repo-sync-action/adr/0004-attestation-strategy.md)
