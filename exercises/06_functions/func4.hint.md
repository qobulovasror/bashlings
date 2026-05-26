# 💡 func4 — Maslahatlar

## 1-bosqich
Bashda **default'da hamma o'zgaruvchilar GLOBAL**. Funksiya ichidagi assignment tashqaridagi shu nomli o'zgaruvchini ham o'zgartiradi:

```bash
fn() {
    name="ichki"      # GLOBAL — buzadi
}
name="tashqi"
fn
echo "$name"          # ichki ← BUG!
```

## 2-bosqich
`local` kalit so'zi — o'zgaruvchini **faqat funksiya ichida** mavjud qiladi:

```bash
fn() {
    local name="ichki"   # ✓ izolyatsiya
}
name="tashqi"
fn
echo "$name"             # tashqi ✓
```

## 3-bosqich
**Best practice:** funksiya ichidagi har yangi o'zgaruvchi uchun `local` ishlating — hatto ulgurmasangiz ham odat qilib qo'ying.
