# 💡 rob5

## Step 1
Checking whether a file exists:
```bash
if [[ -f "$lock" ]]; then
    echo "busy"
    exit 1
fi
```

## Step 2
In production a real lock file is used with a stale check via `kill -0` (see Part 2/05). In this exercise — the simple variant.
