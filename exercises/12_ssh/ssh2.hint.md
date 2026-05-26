# 💡 ssh2

## 1-bosqich
scp sintaksisi:
```
scp <SOURCE> <USER>@<HOST>:<PATH>
```

## 2-bosqich
Lokal manba — oddiy fayl yo'li, prefix kerak emas:
```bash
scp ./file.txt deploy@host:/srv/
```

## 3-bosqich
O'zgaruvchilar bilan:
```bash
echo "scp $SRC $USER@$HOST:$DEST"
```
