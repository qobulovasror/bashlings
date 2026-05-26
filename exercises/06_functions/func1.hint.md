# 💡 func1 — Maslahatlar

## 1-bosqich
Funksiya e'lon qilish sintaksisi:
```bash
greet() {
    echo "salom"
}
```

Yopish — `}`. Boshlash — funksiya nomi + `() {`.

## 2-bosqich
Funksiya ichida argumentlarga `$1`, `$2`, ... orqali murojaat qilinadi:
```bash
greet() {
    echo "Salom, $1!"
}
```

## 3-bosqich
Chaqirish — **qavslarsiz**:
```bash
greet "Ali"        # ✓
greet("Ali")       # ❌ XATO
```
