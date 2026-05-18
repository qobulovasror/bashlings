# 💡 script2 — Maslahatlar

## 1-bosqich
**Command substitution** `$(...)` — buyruq natijasini matn sifatida qaytaradi:
```bash
echo "Bugun: $(date +%F)"
# Bugun: 2026-05-16
```

## 2-bosqich
`$PWD` — environment variable, joriy katalog yo'lini saqlaydi (xuddi `pwd` natijasi kabi).

`basename` — yo'ldan oxirgi qismni qaytaradi:
```bash
basename "/Users/mac/Desktop"   # Desktop
basename "$PWD"                 # joriy katalogning nomi
```

## 3-bosqich
Ikkalasini birlashtirib bitta `echo` ichida ishlating:
```bash
echo "$PWD ($(basename "$PWD"))"
```

## ✅ Yechim
```bash
echo "$PWD ($(basename "$PWD"))"
```
