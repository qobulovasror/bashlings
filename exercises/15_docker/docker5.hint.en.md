# 💡 docker5

## Step 1
Format:
```
<registry>/<owner>/<name>:<tag>
ghcr.io   /myorg /api   :v1.2.3
```

## Step 2
Bash concatenation — variables inside double quotes:
```bash
echo "$REGISTRY/$OWNER/$NAME:$TAG"
```
