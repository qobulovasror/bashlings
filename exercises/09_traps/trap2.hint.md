# 💡 trap2

## 1-bosqich
Funksiya yozish:
```bash
cleanup() {
    echo "tozalandi"
}
```

## 2-bosqich
`trap` ga funksiya nomini berish (inline kod o'rniga):
```bash
trap cleanup EXIT
```

## ✅ Yechim
```bash
cleanup() {
    echo "tozalandi"
}
trap cleanup EXIT
echo "asosiy ish"
```
