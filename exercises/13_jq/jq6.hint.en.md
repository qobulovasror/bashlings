# 💡 jq6

## Step 1
`add` — computes the sum of an array's elements:
```bash
echo '[1,2,3,4]' | jq 'add'
# 10
```

But `add` expects an array — it can't be applied directly to an array of objects.

## Step 2
First extract the array of prices:
```bash
echo '[{"p":1},{"p":2}]' | jq '[.[].p]'
# [1, 2]
```

Then the sum:
```bash
echo '[{"p":1},{"p":2}]' | jq '[.[].p] | add'
# 3
```

## Step 3
Equivalent with `map` too:
```bash
jq 'map(.price) | add'
```
