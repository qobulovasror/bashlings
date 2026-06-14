# 💡 script3 — Hints

## Step 1
The `if` syntax in Bash:
```bash
if [[ shart ]]; then
    # bajariladigan kod
else
    # default
fi
```

`;` — the "newline equivalent" for Linux. You can also put `then` on the next line.

## Step 2
**Numeric comparison operators:**
| Operator | Meaning             |
|----------|---------------------|
| `-eq`    | equal (`==`)        |
| `-ne`    | not equal (`!=`)    |
| `-gt`    | greater than (`>`)  |
| `-lt`    | less than (`<`)     |
| `-ge`    | greater or equal    |
| `-le`    | less or equal       |

⚠ `$yosh > 18` becomes a string comparison — always use `-gt`.

## Step 3
What you need: `[[ $yosh -gt 18 ]]`
