# 💡 trap3

## Step 1
`$?` — the exit code of the very last command. To capture it in cleanup, save it to a variable immediately:
```bash
cleanup() {
    local rc=$?    # the first line is IMPORTANT
    # now the rc value is saved
}
```

## Step 2
If you write `local rc=$?` after other lines — it will hold the exit code of the preceding command, not the one you want.
