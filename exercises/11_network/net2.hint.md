# 💡 net2

## 1-bosqich
Multi-line string'dan FAQAT birinchi qatorni olish:
```bash
echo "$response" | head -1
# HTTP/2 200
```

## 2-bosqich
Bo'sh joy bilan ajratilgan ustunlardan 2-ustunni olish:
```bash
echo "HTTP/2 200" | awk '{print $2}'
# 200
```

`awk` default'da bo'sh joy/tab'larni separator deb qabul qiladi.

## 3-bosqich
Ikkalasini pipe orqali birlashtiramiz:
```bash
echo "$response" | head -1 | awk '{print $2}'
```
