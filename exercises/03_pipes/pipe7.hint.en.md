# 💡 pipe7 — Hints

## Step 1
A plain `>` redirect sends the output **only to the file** — nothing shows up on screen:
```bash
echo "salom" > out.txt   # screen empty, file full
```

## Step 2
`tee` — reads from stdin and writes to both sides (stdout AND the file):
```bash
echo "salom" | tee out.txt
```

## Step 3
`tee -a fayl.txt` — append mode (equivalent to `>>`).
