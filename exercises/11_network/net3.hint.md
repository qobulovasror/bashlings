# 💡 net3

## 1-bosqich
Flag'larni belgilangan tartibda string ichiga qo'ying:
```
curl --retry 3 -m 10 -fsSL <URL>
```

## 2-bosqich
Bash o'zgaruvchisini qo'shtirnoq ichida ishlatish — interpolatsiya bo'ladi:
```bash
URL="https://example.com"
echo "curl --retry 3 -m 10 -fsSL $URL"
```

## 3-bosqich
Eslab qoling: tartib muhim emas curl uchun, lekin **test'da literal taqqoslanadi**,
shuning uchun tavsif'dagi tartibni saqlang.
