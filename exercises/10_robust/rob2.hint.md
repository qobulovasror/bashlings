# 💡 rob2

## 1-bosqich
**Word-splitting tuzog'i**: `name="Ali Vali"` bo'lsa, `$name` tirnoqsiz — 2 ta so'z:
```bash
ls $name        # ls Ali Vali — ikki argument!
```

## 2-bosqich
Qo'shtirnoq ichida — bitta argument:
```bash
ls "$name"      # ls "Ali Vali" — bir argument
```

## 3-bosqich
**Qoida:** har doim `"$var"` shaklida yozing. ShellCheck SC2086 aynan shu uchun.

## ✅ Yechim
```bash
ls "$name"
```
