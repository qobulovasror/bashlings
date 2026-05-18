# 💡 pipe8 — Maslahatlar

## 1-bosqich
**Here-string** (`<<<`) — bitta qator matnni buyruqning **stdin**'iga uzatadi:
```bash
cat <<< "salom"   # "salom" chiqaradi
```

## 2-bosqich
`bc` (Basic Calculator) — kalkulyator. Stdin'dan ifoda kutadi va natijani chiqaradi:
```bash
echo "3+4" | bc   # 7
```

## 3-bosqich
Ikkalasini birlashtirib, here-string'dan foydalaning:
```bash
bc <<< "3+4"
```

## ✅ Yechim
```bash
bc <<< "5+7"
```
