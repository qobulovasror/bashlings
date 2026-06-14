# 💡 func4 — Hints

## Step 1
In Bash **all variables are GLOBAL by default**. An assignment inside a function also changes a variable of the same name outside:

```bash
fn() {
    name="ichki"      # GLOBAL — breaks it
}
name="tashqi"
fn
echo "$name"          # ichki ← BUG!
```

## Step 2
The `local` keyword makes a variable exist **only inside the function**:

```bash
fn() {
    local name="ichki"   # ✓ isolated
}
name="tashqi"
fn
echo "$name"             # tashqi ✓
```

## Step 3
**Best practice:** use `local` for every new variable inside a function — make it a habit even when you think you don't need it.
