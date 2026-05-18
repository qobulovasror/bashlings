# 💡 arr5

## 1-bosqich
**Associative array** Bash 4+'da. macOS stock bash 3.2 ishlamaydi:
```bash
bash --version   # 4.x yoki 5.x bo'lishi kerak
brew install bash  # agar 3.x bo'lsa
```

## 2-bosqich
E'lon qilish va to'ldirish:
```bash
declare -A user
user[name]="Ali"
user[city]="Toshkent"
```

Yoki bir martda:
```bash
declare -A user=([name]="Ali" [city]="Toshkent")
```

## ✅ Yechim
```bash
declare -A user
user[name]="Ali"
user[city]="Toshkent"
echo "${user[name]} ${user[city]}"
```
