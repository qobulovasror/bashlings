# 💡 jq3

## Step 1
`select(condition)` — keeps the ones that match the condition:
```bash
echo '[{"a":1},{"a":2},{"a":3}]' | jq '.[] | select(.a > 1)'
# {"a":2}
# {"a":3}
```

## Step 2
`length` — the length of an array. But `.[]` returns multiple outputs, so `length` won't work. **Wrap it in an array**:
```bash
echo '[1,2,3]' | jq '[.[] | select(. > 1)] | length'
# 2
```

## Step 3
Or via `map` — equivalent:
```bash
echo '[1,2,3]' | jq 'map(select(. > 1)) | length'
```
