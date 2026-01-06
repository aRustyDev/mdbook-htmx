---
title: Ingress Authentication
description: Add authentication with OAuth2 Proxy
---

# Ingress Authentication

Add authentication using OAuth2 Proxy or Basic Auth.

## OAuth2 Proxy

### Deploy OAuth2 Proxy

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: oauth2-proxy
spec:
  selector:
    matchLabels:
      app: oauth2-proxy
  template:
    metadata:
      labels:
        app: oauth2-proxy
    spec:
      containers:
        - name: oauth2-proxy
          image: quay.io/oauth2-proxy/oauth2-proxy:latest
          args:
            - --provider=oidc
            - --oidc-issuer-url=https://auth.example.com
            - --upstream=http://docs:80
            - --http-address=0.0.0.0:4180
          env:
            - name: OAUTH2_PROXY_CLIENT_ID
              valueFrom:
                secretKeyRef:
                  name: oauth2-secrets
                  key: client-id
            - name: OAUTH2_PROXY_CLIENT_SECRET
              valueFrom:
                secretKeyRef:
                  name: oauth2-secrets
                  key: client-secret
```

### Ingress with Auth

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: docs
  annotations:
    nginx.ingress.kubernetes.io/auth-url: "http://oauth2-proxy.default.svc.cluster.local:4180/oauth2/auth"
    nginx.ingress.kubernetes.io/auth-signin: "https://docs.example.com/oauth2/start"
spec:
  rules:
    - host: docs.example.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: docs
                port:
                  number: 80
```

## Basic Auth

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: docs
  annotations:
    nginx.ingress.kubernetes.io/auth-type: basic
    nginx.ingress.kubernetes.io/auth-secret: basic-auth
    nginx.ingress.kubernetes.io/auth-realm: "Documentation"
spec:
  # ... ingress spec
```

## See Also

- [Authentication Feature](../../features/authentication.md)
- [With Meilisearch](with-meilisearch.md)
