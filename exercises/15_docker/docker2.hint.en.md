# 💡 docker2

## Step 1
Heredoc — printing multi-line text:
```bash
cat <<EOF
line1
line2
EOF
```

## Step 2
Dockerfile directives (written literally):
```
FROM python:3.11-alpine
WORKDIR /app
COPY app.py .
CMD ["python", "app.py"]
```

The `"` inside `CMD` — no bash interpolation, because the heredoc prints
line by line.
