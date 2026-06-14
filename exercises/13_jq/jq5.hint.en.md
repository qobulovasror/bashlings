# 💡 jq5

## Step 1
`max_by(filter)` — returns the largest element from an array (according to `filter`):
```bash
echo '[{"x":3},{"x":1},{"x":5}]' | jq 'max_by(.x)'
# {"x": 5}
```

There is also `min_by` — the smallest.

## Step 2
Extract the field you need from the resulting object — via `|`:
```bash
echo '[{"x":3,"n":"A"},{"x":5,"n":"B"}]' | jq 'max_by(.x) | .n'
# "B"
```

## Step 3
Always use `-r` — raw output.
