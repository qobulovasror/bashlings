# 💡 script6 — Hints

## Step 1
`while` — repeats as long as the condition is **TRUE**:
```bash
i=1
while [[ $i -le 3 ]]; do
    echo "Qadam $i"
    ((i++))
done
```

## Step 2
`((i++))` — arithmetic context — equals `i = i + 1` (shorter).

Alternatives:
- `i=$((i + 1))`
- `let i=i+1`
- `((i += 1))`

## Step 3
⚠ Don't forget to increment the counter — otherwise you get an **infinite loop**!
