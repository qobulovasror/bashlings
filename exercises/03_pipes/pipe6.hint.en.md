# 💡 pipe6 — Hints

## Step 1
`sort` — sorts lines in alphabetical order.

## Step 2
`uniq` — removes **consecutive** duplicates. That is why it is always used in the order `sort | uniq`.

```bash
cat fruits.txt | sort | uniq
```

Or shorter (passing the file as an argument):
```bash
sort fruits.txt | uniq
```

## Step 3
**Bonus:** `sort -u` does exactly this same task in a single command.
