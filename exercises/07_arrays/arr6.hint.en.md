# 💡 arr6

## Step 1
`read -ra` — split a string into an array:
- `-r` — don't process backslash escapes (always use it)
- `-a names` — write the result into the `names` array

## Step 2
`IFS=','` — split on commas:
```bash
IFS=',' read -ra names <<< "$csv"
```

`<<<` — a here-string (single-line stdin).

`IFS=','` applies only to this `read` — it doesn't affect the global value.
