# 💡 docker7

## Step 1
YAML indentation is 2 spaces per level. Example:
```yaml
parent:
  child:
    grandchild: value
```

## Step 2
Quoting the port — in YAML it is written as `"8080:80"` (so it is not
interpreted as a number).

## Step 3
Via heredoc — each line with EXACTLY this indentation:
```bash
cat <<EOF
services:
  web:
    image: nginx:alpine
    ports:
      - "8080:80"
    environment:
      ENV: production
EOF
```
