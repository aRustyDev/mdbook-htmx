# CI/CD Workflows

This directory documents the GitHub Actions workflows used in mdbook-htmx.

## Workflows Overview

| Workflow | File | Purpose | Triggers |
|----------|------|---------|----------|
| [CI](./ci.md) | `ci.yml` | Code quality checks | Push/PR to main |
| [Release](./release.md) | `release.yml` | Build and publish releases | Version tags |
| [Test Deployments](./test-deployments.md) | `test-deployments.yml` | Test deployment configs | Changes to docker/workers |
| [Deploy Cloudflare](./deploy-cloudflare.md) | `deploy-cloudflare.yml` | Deploy to Cloudflare | Main branch push |
| [Cross-Repo Sync](./cross-repo-sync.md) | `docs-sync.yml` | Sync docs to central repo | Push to main (docs/**) |

## Workflow Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         CI/CD PIPELINE                                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Pull Request                    Main Branch                  Tags       │
│       │                               │                         │        │
│       ▼                               ▼                         ▼        │
│  ┌─────────┐                    ┌─────────┐               ┌─────────┐   │
│  │   CI    │                    │   CI    │               │ Release │   │
│  │ (check, │                    │ (check, │               │ (build, │   │
│  │  test,  │                    │  test,  │               │ publish)│   │
│  │  fmt,   │                    │  fmt,   │               └────┬────┘   │
│  │ clippy) │                    │ clippy) │                    │        │
│  └─────────┘                    └────┬────┘                    │        │
│                                      │                         │        │
│                           ┌──────────┴──────────┐              │        │
│                           ▼                     ▼              │        │
│                    ┌──────────────┐     ┌──────────────┐       │        │
│                    │   Deploy     │     │  Docs Sync   │       │        │
│                    │ (Cloudflare) │     │ (to central) │       │        │
│                    └──────────────┘     └──────────────┘       │        │
│                                                                │        │
│  ┌──────────────────────────────────────────────────────────────────┐   │
│  │                     Test Deployments                              │   │
│  │  (Docker Compose, Helm Verify, Helm Install, Cloudflare Types)   │   │
│  └──────────────────────────────────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## Security Features

### Cross-Repo Sync Attestation

The docs-sync workflow creates cryptographic attestations for all sync operations:

- **Sigstore Signing**: Keyless signing via GitHub OIDC
- **In-toto Format**: Industry-standard attestation format
- **Audit Trail**: Records source, target, files synced, and xauth metadata
- **Verification**: Attestations can be verified using `gh attestation verify`

See [Cross-Repo Sync](./cross-repo-sync.md) for details.

### Helm Chart Verification

The test-deployments workflow includes Cosign signature verification for helm charts:

- **Signature Verification**: Validates chart signatures from `ghcr.io/arustydev/charts`
- **OIDC Identity**: Uses GitHub Actions OIDC for keyless signing
- **Provenance**: Checks for SLSA provenance attestations

See [ADR-0008: Helm Chart Verification](../src/adr/0008-helm-chart-verification.md) for details.

## Related Documentation

- [Cross-Repo Sync Action](/.github/actions/cross-repo-sync/README.md)
- [ADR-0008: Helm Chart Verification](../src/adr/0008-helm-chart-verification.md)
- [Kubernetes Deployment](../src/deployment/kubernetes/helm.md)
- [Cloudflare Deployment](../src/deployment/cloudflare/workers.md)
