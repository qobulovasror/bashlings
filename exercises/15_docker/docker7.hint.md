# 💡 docker7

## 1-bosqich
YAML chekinishi 2-spaces, har bosqichda. Misol:
```yaml
parent:
  child:
    grandchild: value
```

## 2-bosqich
Port'ning quotes'i — YAML'da `"8080:80"` ko'rinishida (number sifatida talqin
qilinmasligi uchun).

## 3-bosqich
Heredoc orqali — har qator AYNAN shu chekinish bilan:
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
