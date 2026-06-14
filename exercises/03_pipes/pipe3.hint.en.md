# 💡 pipe3 — Hints

## Step 1
The `|` (pipe) operator — passes the **stdout** of the first command into the **stdin** of the second command.

```bash
echo "salom" | wc -c
```

## Step 2
`tr` — a command that replaces characters. Syntax:
```bash
tr <eski_belgi> <yangi_belgi>
```

`'\n'` — the newline character. It must be inside quotes.

## Step 3
What you need: replace a space (' ') with a newline ('\n').
