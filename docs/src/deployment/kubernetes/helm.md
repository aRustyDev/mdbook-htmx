---
title: Helm Chart
description: Deploy with Helm chart
---

# Helm Chart

Use the official Helm chart for parameterized deployment.

## Installation

```bash
# Add the repository
helm repo add arustydev https://charts.arusty.dev
helm repo update

# Install
helm install docs arustydev/mdbook-htmx \
  --set ingress.enabled=true \
  --set ingress.host=docs.example.com
```

## values.yaml

```yaml
replicaCount: 3

image:
  repository: ghcr.io/arustydev/mdbook-htmx
  tag: latest

ingress:
  enabled: true
  host: docs.example.com
  annotations:
    kubernetes.io/ingress.class: nginx

resources:
  limits:
    cpu: 100m
    memory: 128Mi
  requests:
    cpu: 50m
    memory: 64Mi

autoscaling:
  enabled: true
  minReplicas: 2
  maxReplicas: 10
  targetCPUUtilizationPercentage: 80
```

## Customization

Override values:

```bash
helm install docs arustydev/mdbook-htmx -f my-values.yaml
```

## Upgrade

```bash
helm upgrade docs arustydev/mdbook-htmx -f my-values.yaml
```

## See Also

- [Basic Deployment](basic.md) - Raw manifests
- [Ingress Authentication](ingress-auth.md) - Add auth
