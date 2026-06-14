# 💡 func6 — Hints

## Step 1
**Boolean function** — done reliably with `return`:
- `return 0` = "yes, true" (success)
- `return 1` (or any non-zero) = "no, false" (fail)

Note: this is the **Unix convention** — 0 means success, non-zero means error. Not the other way around!

## Step 2
Checking for even — `% 2`:
- `4 % 2` = 0 → even
- `7 % 2` = 1 → odd

```bash
is_even() {
    return $(( $1 % 2 ))
}
```

## Step 3
In an `if` condition a function is checked by its **exit code**:
```bash
if is_even 4; then
    echo "juft"
fi
```
