# 💡 rob3

## Step 1
**Parameter expansion** — `${VAR:-default}`:
- VAR empty/unset → default
- VAR set → its own value

## Step 2
```bash
level="${LOG_LEVEL:-info}"
```

Usage:
```bash
./script.sh                # Level: info
LOG_LEVEL=debug ./script.sh # Level: debug
```
