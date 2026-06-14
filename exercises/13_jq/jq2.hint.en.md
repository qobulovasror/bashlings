# 💡 jq2

## Step 1
`.[]` — outputs the array elements **one by one** (multi-output).

```bash
echo '[1,2,3]' | jq '.[]'
# 1
# 2
# 3
```

## Step 2
Access each element's field with `.field`:
```bash
echo '[{"a":1},{"a":2}]' | jq '.[].a'
# 1
# 2
```

## Step 3
Always use `-r`, to remove the quotes.
