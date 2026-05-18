# 💡 script1 — Maslahatlar

## 1-bosqich
O'zgaruvchini e'lon qilishda `=` atrofida **probel bo'lmasligi kerak**:
```bash
name="Bash"      # ✅ to'g'ri
name = "Bash"    # ❌ XATO — bash buni buyruq deb tushunadi
```

## 2-bosqich
"..." qo'shtirnoq ichida `$o'zgaruvchi` avtomatik almashtiriladi:
```bash
echo "Salom, $name!"   # Salom, Bash!
```

'...' yagona tirnoq ichida almashtirilmaydi:
```bash
echo 'Salom, $name!'   # Salom, $name! — literal
```

## ✅ Yechim
```bash
name="Bash"
echo "Salom, $name!"
```
