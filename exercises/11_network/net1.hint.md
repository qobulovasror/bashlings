# 💡 net1

## 1-bosqich
Bash buyruq nomi NOTo'g'ri yozilgan. `eko` emas, `echo`.

## 2-bosqich
`echo "..."` ichida `$URL` o'zgaruvchisi avtomatik interpolatsiya bo'ladi
(qo'shtirnoq ikki tomonlama, bir tirnoq emas):
```bash
URL="https://x.io"
echo "curl -fsSL $URL"
# curl -fsSL https://x.io
```

## 3-bosqich
Flag'lar tartibi: `-f -s -S -L` → birlashtirish: `-fsSL`. Bash uchun bir xil.
