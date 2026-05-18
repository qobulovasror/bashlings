# 💡 rob5

## 1-bosqich
Fayl mavjudligini tekshirish:
```bash
if [[ -f "$lock" ]]; then
    echo "busy"
    exit 1
fi
```

## 2-bosqich
Production'da haqiqiy lock fayli `kill -0` bilan stale tekshiruvi bilan ishlatiladi (Part 2/05 ko'ring). Bu mashqda — sodda variant.

## ✅ Yechim
```bash
if [[ -f "$lock" ]]; then
    echo "busy"
    exit 1
fi
echo "ish bajarilmoqda"
```
