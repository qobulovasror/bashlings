# 💡 ssh2

## Step 1
scp syntax:
```
scp <SOURCE> <USER>@<HOST>:<PATH>
```

## Step 2
The local source is just a plain file path, no prefix needed:
```bash
scp ./file.txt deploy@host:/srv/
```

## Step 3
With variables:
```bash
echo "scp $SRC $USER@$HOST:$DEST"
```
