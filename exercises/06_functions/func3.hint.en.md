# 💡 func3 — Hints

## Step 1
**Parameter expansion** `${var:-default}`:
- If `var` is empty or unset — returns `default`
- Otherwise — the value of `var`

```bash
echo "${1:-Anonim}"
# $1 given: value of $1
# $1 not given: Anonim
```

## Step 2
Combine this syntax with `local`:
```bash
greet() {
    local name="${1:-Anonim}"
    echo "Salom, $name!"
}
```

## Step 3
**Similar variants:**
- `${var:-default}` — default value (var stays unchanged)
- `${var:=default}` — default and also sets var
- `${var:?xato}` — exits with an error if empty
- `${var:+almashtir}` — if not empty, the replacement value
