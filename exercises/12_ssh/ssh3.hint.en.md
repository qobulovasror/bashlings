# 💡 ssh3

## Step 1
ssh-keygen command template:
```
ssh-keygen -t <algo> -C "<comment>" -f <path>
```

## Step 2
- `-t ed25519` algorithm
- `-C "email"` — comment inside **double quotes**
- `-f path` — file path

## Step 3
With variables:
```bash
echo "ssh-keygen -t ed25519 -C \"$EMAIL\" -f $PATH_KEY"
```

Escaping `"` inside `echo "..."`: `\"`.
