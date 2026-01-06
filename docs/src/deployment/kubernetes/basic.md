---
title: Kubernetes Basic
description: Basic Kubernetes deployment
---

# Kubernetes Basic Deployment

Deploy mdbook-htmx to a Kubernetes cluster.

## Manifests

### Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: docs
spec:
  replicas: 3
  selector:
    matchLabels:
      app: docs
  template:
    metadata:
      labels:
        app: docs
    spec:
      containers:
        - name: docs
          image: ghcr.io/arustydev/mdbook-htmx:latest
          ports:
            - containerPort: 3000
          env:
            - name: DOCS_ROOT
              value: /app/book/htmx
          volumeMounts:
            - name: docs-content
              mountPath: /app/book
      volumes:
        - name: docs-content
          configMap:
            name: docs-content
```

### Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: docs
spec:
  selector:
    app: docs
  ports:
    - port: 80
      targetPort: 3000
```

### Ingress

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: docs
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

## Deploy

```bash
kubectl apply -f k8s/
```

## See Also

- [Helm Chart](helm.md) - Parameterized deployment
- [Ingress Authentication](ingress-auth.md) - Add authentication
