# 💡 pipe8 — Hints

## Step 1
A **here-string** (`<<<`) — passes a single line of text into a command's **stdin**:
```bash
cat <<< "salom"   # prints "salom"
```

## Step 2
`bc` (Basic Calculator) — a calculator. It expects an expression from stdin and prints the result:
```bash
echo "3+4" | bc   # 7
```

## Step 3
Combine the two and use a here-string:
```bash
bc <<< "3+4"
```
