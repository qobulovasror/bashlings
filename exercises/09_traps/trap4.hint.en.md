# 💡 trap4

## Step 1
Two actions inside cleanup:
1. Delete the file: `rm -f "$tmpfile"`
2. Print a message: `echo "tmp deleted"`

The `-f` flag — don't error even if the file doesn't exist.

## Step 2
```bash
cleanup() {
    rm -f "$tmpfile"
    echo "tmp deleted"
}
trap cleanup EXIT
```
