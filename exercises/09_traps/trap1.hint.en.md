# 💡 trap1

## Step 1
`trap` syntax:
```bash
trap '<code>' SIGNAL
```

EXIT — a Bash pseudo-signal, runs **in all cases** (normal exit, error, signal, ...).

## Step 2
```bash
trap 'echo "cleaned up"' EXIT
```
