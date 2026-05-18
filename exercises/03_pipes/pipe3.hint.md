# 💡 pipe3 — Maslahatlar

## 1-bosqich
`|` (pipe) operatori — birinchi buyruqning **stdout**'ini ikkinchi buyruqning **stdin**'iga uzatadi.

```bash
echo "salom" | wc -c
```

## 2-bosqich
`tr` — belgilarni almashtiruvchi buyruq. Sintaksisi:
```bash
tr <eski_belgi> <yangi_belgi>
```

`'\n'` — yangi qator (newline) belgisi. Qo'shtirnoq ichida bo'lishi shart.

## 3-bosqich
Sizga kerak: bo'shliqni (' ') yangi qatorga ('\n') almashtirish.

## ✅ Yechim
```bash
echo "salom dunyo bash" | tr ' ' '\n'
```
