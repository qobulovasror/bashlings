# 💡 intro4 — Maslahatlar

## 1-bosqich
`date` buyrug'i sukut bo'yicha uzun format chiqaradi (masalan: `Sat May 16 14:22:01 2026`). Bizga **YYYY-MM-DD** kerak.

## 2-bosqich
Bash dokumentatsiyasida `date +%F` — bu **ISO 8601** formati: `2026-05-16`. Sinab ko'ring terminalda.

## 3-bosqich
Buyruq natijasini o'zgaruvchi orqali ishlatish uchun `$(...)` ishlatiladi:
```bash
today=$(date +%F)
echo "Bugun: $today"
```
Yoki bitta qatorda:
```bash
echo "Bugun: $(date +%F)"
```

## ✅ Yechim
```bash
echo "Bugun: $(date +%F)"
```
