# 💡 awk1

## Step 1
`awk` automatically splits each line into **fields** on whitespace:
- `$0` — the whole line
- `$1`, `$2`, `$3` — the N-th field
- `$NF` — the last field

## Step 2
The 3rd column (city) — `$3`:
```bash
awk '{print $3}' data.txt
```
