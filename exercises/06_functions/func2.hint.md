# 💡 func2 — Maslahatlar

## 1-bosqich
Bashda `return` faqat **exit code (0-255)** uchun. Haqiqiy qiymatni `echo` orqali qaytariladi:
```bash
double() {
    echo $(( $1 * 2 ))
}
```

## 2-bosqich
Chaqiruvchi tomonda `$(...)` orqali ushlanadi:
```bash
r=$(double 5)
echo "$r"        # 10
```

## 3-bosqich
Arifmetika: `$((expression))` — ichida `*`, `+`, `-`, `/`, `%` ishlatish mumkin.

Kvadrat: `$1 * $1` yoki `$1 ** 2`.

## ✅ Yechim
```bash
square() {
    echo $(( $1 * $1 ))
}

result=$(square 5)
echo "5 ning kvadrati: $result"
```
