# 💡 text2 — Hints

## Step 1
`grep -i` — **case-insensitive** mode. `error`, `Error`, `ERROR` — it finds all three.

```bash
grep -i "error" log.txt   # finds 3 lines
```

## Step 2
`grep -c` — prints the **number** of matches (not the matched text):
```bash
grep -c "error" log.txt   # 3
```

## Step 3
You can combine the two flags:
```bash
grep -ic "error" log.txt
```
