# 💡 docker2

## 1-bosqich
Heredoc — multi-line matnni chiqarish:
```bash
cat <<EOF
qator1
qator2
EOF
```

## 2-bosqich
Dockerfile direktivalari (literal yoziladi):
```
FROM python:3.11-alpine
WORKDIR /app
COPY app.py .
CMD ["python", "app.py"]
```

`CMD` ichidagi `"` — bash interpolatsiyasi yo'q, chunki heredoc qator-by-qator
shaklda chiqaradi.
