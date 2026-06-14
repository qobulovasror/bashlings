# 💡 text5 — Hints

## Step 1
We solve "take lines 5-7" in two steps:
1. Take the first **7** lines — `head -n 7`
2. The **last 3** lines (5, 6, 7) of that result — `tail -n 3`

## Step 2
Chain the two with `|` (pipe):
```bash
head -n 7 big.txt | tail -n 3
```

Mental model:
```
big.txt: 1 2 3 4 5 6 7 8 9 ... 100
   ↓ head -n 7
         1 2 3 4 5 6 7
                  ↓ tail -n 3
                       5 6 7
```

## Step 3
**Bonus** — this pattern is used very often:
- lines 50-100: `head -n 100 fayl | tail -n 51`
- Or with `sed`: `sed -n '5,7p' big.txt` (sed is covered in Part 2)
