# 💡 func6 — Maslahatlar

## 1-bosqich
**Boolean funksiya** — `return` orqali ishonchli:
- `return 0` = "ha, rost" (success)
- `return 1` (yoki boshqa nol bo'lmagan) = "yo'q, yolg'on" (fail)

Diqqat: bu **Unix konvensiyasi** — 0 muvaffaqiyat, nol bo'lmagani xato. Aksincha **emas**!

## 2-bosqich
Juftlikni tekshirish — `% 2`:
- `4 % 2` = 0 → juft
- `7 % 2` = 1 → toq

```bash
is_even() {
    return $(( $1 % 2 ))
}
```

## 3-bosqich
`if` shartda funksiya **exit code** bo'yicha tekshiriladi:
```bash
if is_even 4; then
    echo "juft"
fi
```

## ✅ Yechim
```bash
is_even() {
    return $(( $1 % 2 ))
}
```

Yoki aniq:
```bash
is_even() {
    if (( $1 % 2 == 0 )); then
        return 0
    else
        return 1
    fi
}
```
