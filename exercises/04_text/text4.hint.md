# 💡 text4 — Maslahatlar

## 1-bosqich
`cut` — qatordan ustun (field) ajratish uchun.

Sintaksis: `cut -d <separator> -f <field_number>`

- `-d ','` — vergul **delimiter** (ajratuvchi)
- `-f 1`   — 1-ustunni olish (1'dan boshlanadi, 0'dan emas)

## 2-bosqich
Misol:
```bash
echo "ali,25,toshkent" | cut -d ',' -f 1
# ali

echo "ali,25,toshkent" | cut -d ',' -f 2
# 25

echo "ali,25,toshkent" | cut -d ',' -f 1,3
# ali,toshkent
```

## 3-bosqich
Fayl bilan ham ishlaydi:
```bash
cut -d',' -f1 users.csv
```

## ✅ Yechim
```bash
cut -d ',' -f 1 users.csv
```
