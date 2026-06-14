# 💡 jq1

## Step 1
`jq` syntax: `jq '.field'` — get a field from an object.

## Step 2
By default `jq` outputs with quotes:
```bash
echo '{"name":"Ali"}' | jq '.name'      # "Ali"
echo '{"name":"Ali"}' | jq -r '.name'   # Ali — raw
```

In a shell variable, or for testing, always use `-r`.
