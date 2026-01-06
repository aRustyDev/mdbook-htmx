---
title: Kubernetes with Meilisearch
description: Add Meilisearch for server-side search
---

# Kubernetes with Meilisearch

Deploy Meilisearch alongside mdbook-htmx for server-side search.

## Meilisearch Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: meilisearch
spec:
  selector:
    matchLabels:
      app: meilisearch
  template:
    metadata:
      labels:
        app: meilisearch
    spec:
      containers:
        - name: meilisearch
          image: getmeili/meilisearch:v1.6
          ports:
            - containerPort: 7700
          env:
            - name: MEILI_MASTER_KEY
              valueFrom:
                secretKeyRef:
                  name: meilisearch-secrets
                  key: master-key
          volumeMounts:
            - name: meili-data
              mountPath: /meili_data
      volumes:
        - name: meili-data
          persistentVolumeClaim:
            claimName: meilisearch-pvc
```

## Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: meilisearch
spec:
  selector:
    app: meilisearch
  ports:
    - port: 7700
```

## Docs Configuration

Update docs deployment:

```yaml
env:
  - name: MEILISEARCH_URL
    value: http://meilisearch:7700
  - name: MEILISEARCH_KEY
    valueFrom:
      secretKeyRef:
        name: meilisearch-secrets
        key: api-key
```

## Index Update Job

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: update-search-index
spec:
  template:
    spec:
      containers:
        - name: updater
          image: curlimages/curl
          command:
            - /bin/sh
            - -c
            - |
              curl -X POST "$MEILISEARCH_URL/indexes/docs/documents" \
                -H "Authorization: Bearer $MEILISEARCH_KEY" \
                -H "Content-Type: application/json" \
                --data-binary @/data/search-index.json
      restartPolicy: Never
```

## See Also

- [Search Feature](../../features/search.md)
- [Adding Search Tutorial](../../tutorials/adding-search.md)
