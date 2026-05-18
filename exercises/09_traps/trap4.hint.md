# 💡 trap4

## 1-bosqich
Cleanup ichida 2 ta amal:
1. Faylni o'chirish: `rm -f "$tmpfile"`
2. Xabar chiqarish: `echo "tmp o'chirildi"`

`-f` flagi — fayl yo'q bo'lsa ham xato bermaslik.

## 2-bosqich
```bash
cleanup() {
    rm -f "$tmpfile"
    echo "tmp o'chirildi"
}
trap cleanup EXIT
```

## ✅ Yechim
```bash
cleanup() {
    rm -f "$tmpfile"
    echo "tmp o'chirildi"
}
trap cleanup EXIT
echo "asosiy ish"
```
