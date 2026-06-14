# 💡 cron6

## Step 1
Splitting fields with `cut`:
```bash
echo "a b c d e f g" | cut -d' ' -f6
# f
```

- `-d' '` — delimiter is a space
- `-f6`   — the 6th field

## Step 2
EVERYTHING FROM THE 6TH FIELD TO THE END:
```bash
echo "a b c d e f g" | cut -d' ' -f6-
# f g
```

`-f6-` — "from 6 to the end".

## Step 3
In our line there are 5 time fields + the command:
```bash
echo "$line" | cut -d' ' -f6-
```
