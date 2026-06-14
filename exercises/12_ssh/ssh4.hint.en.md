# 💡 ssh4

## Step 1
Heredoc — output a multi-line string with interpolation:
```bash
cat <<EOF
Host $HOST_ALIAS
    HostName $HOSTNAME
EOF
```

## Step 2
Note: indentation — each property line starts with EXACTLY 4 spaces
(not a tab).

## Step 3
Each line inside the heredoc is written literally (not bash syntax):
```bash
cat <<EOF
Host $HOST_ALIAS
    HostName $HOSTNAME
    User $USER
    Port $PORT
    IdentityFile $KEY
EOF
```
