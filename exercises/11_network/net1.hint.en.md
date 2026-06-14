# 💡 net1

## Step 1
The Bash command name is spelled INCORRECTLY. Not `eko`, but `echo`.

## Step 2
Inside `echo "..."` the `$URL` variable is interpolated automatically
(double quotes, not single quotes):
```bash
URL="https://x.io"
echo "curl -fsSL $URL"
# curl -fsSL https://x.io
```

## Step 3
Flag order: `-f -s -S -L` → combined: `-fsSL`. The same for Bash.
