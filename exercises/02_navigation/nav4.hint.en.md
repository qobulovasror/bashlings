# 💡 nav4 — Hints

## Step 1
**Brace expansion** — the shell automatically expands `{a,b,c}` into `a b c` (separated by spaces).

## Step 2
Examples:
- `echo {1,2,3}` → `1 2 3`
- `touch file{1,2,3}.txt` → creates file1.txt, file2.txt, file3.txt

## Step 3
A range also works: `{1..3}` behaves just like `{1,2,3}`.
