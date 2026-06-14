# 💡 func1 — Hints

## Step 1
Function declaration syntax:
```bash
greet() {
    echo "salom"
}
```

Close with `}`. Open with the function name + `() {`.

## Step 2
Inside a function arguments are accessed via `$1`, `$2`, ...:
```bash
greet() {
    echo "Salom, $1!"
}
```

## Step 3
Call it **without parentheses**:
```bash
greet "Ali"        # ✓
greet("Ali")       # ❌ WRONG
```
