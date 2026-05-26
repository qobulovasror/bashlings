# 💡 ssh4

## 1-bosqich
Heredoc — multi-line stringni interpolatsiya bilan chiqarish:
```bash
cat <<EOF
Host $HOST_ALIAS
    HostName $HOSTNAME
EOF
```

## 2-bosqich
Diqqat: chekinish — har property satri AYNAN 4 ta bo'sh joy bilan boshlanadi
(tab emas).

## 3-bosqich
Heredoc ichidagi har qator literal yoziladi (bash sintaksisi emas):
```bash
cat <<EOF
Host $HOST_ALIAS
    HostName $HOSTNAME
    User $USER
    Port $PORT
    IdentityFile $KEY
EOF
```
