# 💡 script5 — Hints

## Step 1
`case` — an elegant alternative to `if/elif/elif/else`. It is cleaner when comparing one variable against many values.

## Step 2
Syntax:
```bash
case "$variable" in
    "a")
        echo "Birinchi"
        ;;
    "b")
        echo "Ikkinchi"
        ;;
    *)
        echo "Boshqa"
        ;;
esac
```

Note:
- Each branch ends with `;;` (two semicolons)
- `*)` — the default branch, comes last
- `esac` — `case` spelled backwards (the closing keyword)

## Step 3
`tugma` = "b" → falls into the "Ikkinchi" branch.
