# 💡 sed2

## Step 1
`sed 'Nd'` — deletes line N. Example:
```bash
sed '1d' file       # line 1 is deleted
sed '5,10d' file    # range 5-10
```

## Step 2
To delete several individual lines — via `;`:
```bash
sed '2d;4d' log.txt
```

Or with `-e`:
```bash
sed -e '2d' -e '4d' log.txt
```
