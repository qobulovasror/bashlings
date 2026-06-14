# 💡 trap2

## Step 1
Writing a function:
```bash
cleanup() {
    echo "cleaned up"
}
```

## Step 2
Give `trap` the function name (instead of inline code):
```bash
trap cleanup EXIT
```
