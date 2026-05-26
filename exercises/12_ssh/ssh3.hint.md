# 💡 ssh3

## 1-bosqich
ssh-keygen komandasi shabloni:
```
ssh-keygen -t <algo> -C "<comment>" -f <path>
```

## 2-bosqich
- `-t ed25519` algoritm
- `-C "email"` — komment **qo'shtirnoq** ichida
- `-f path` — fayl yo'li

## 3-bosqich
O'zgaruvchilar:
```bash
echo "ssh-keygen -t ed25519 -C \"$EMAIL\" -f $PATH_KEY"
```

`echo "..."` ichida `"` ni escape qilish: `\"`.
