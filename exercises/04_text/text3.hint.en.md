# 💡 text3 — Hints

## Step 1
**The most important concept:** by default `sort` sorts as text (lexicographic):
```bash
echo $'100\n23\n5' | sort
# 100
# 23
# 5     ← wrong order for numbers!
```

The reason: `'1'` < `'2'` < `'5'` (it compares by the first character).

## Step 2
The `-n` flag — **numeric sort**. It sorts numbers by their value:
```bash
echo $'100\n23\n5' | sort -n
# 5
# 23
# 100
```
