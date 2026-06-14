# 💡 func2 — Hints

## Step 1
In Bash `return` is only for an **exit code (0-255)**. A real value is returned via `echo`:
```bash
double() {
    echo $(( $1 * 2 ))
}
```

## Step 2
On the caller side it is captured with `$(...)`:
```bash
r=$(double 5)
echo "$r"        # 10
```

## Step 3
Arithmetic: `$((expression))` — you can use `*`, `+`, `-`, `/`, `%` inside.

Square: `$1 * $1` or `$1 ** 2`.
