# 💡 func5 — Hints

## Step 1
Each function returns its result via `echo`:
```bash
add() {
    echo $(( $1 + $2 ))
}

multiply() {
    echo $(( $1 * $2 ))
}
```

## Step 2
Passing one function's result into another — via **command substitution**:
```bash
result=$(add 3 4)            # result = 7
final=$(multiply "$result" 2) # final = 14
```

## Step 3
Or on a single line (but harder to read):
```bash
echo $(multiply "$(add 3 4)" 2)
```
