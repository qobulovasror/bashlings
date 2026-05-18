# 💡 awk3

## 1-bosqich
**Counter pattern** — awk associative array bilan:
```bash
awk '{count[$1]++} END {for (k in count) print k, count[k]}'
```

`$1` — har qatorning birinchi field. `count[$1]++` — shu field uchun counter oshiradi.

## 2-bosqich
Awk associative array tartibi **non-deterministic**. Aniq tartib uchun pipe orqali sort:
```bash
awk '{count[$1]++} END {for (k in count) print k, count[k]}' log.txt | sort
```

## 3-bosqich
Bu pattern DevOps'da eng ko'p ishlatiladigan idioma — log statistikasi, IP frequency va h.k.

## ✅ Yechim
```bash
awk '{count[$1]++} END {for (k in count) print k, count[k]}' log.txt | sort
```
