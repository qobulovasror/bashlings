# 💡 awk2

## 1-bosqich
**Accumulate pattern** — har qatorda o'sib boruvchi o'zgaruvchi:
```bash
awk '{sum += $2}' file
```

awk'da o'zgaruvchini e'lon qilish shart emas — birinchi ishlatishda 0 deb hisoblanadi.

## 2-bosqich
Yakuniy natijani chiqarish — `END` bloki:
```bash
awk '{sum += $2} END {print sum}' scores.txt
```

`END { }` barcha qatorlar o'qib bo'lingach ishlaydi.

## 3-bosqich
O'rtacha hisoblash uchun:
```bash
awk '{sum += $2; n++} END {print sum/n}'
```

## ✅ Yechim
```bash
awk '{sum += $2} END {print sum}' scores.txt
```
