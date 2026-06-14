# 💡 text4 — Hints

## Step 1
`cut` — for extracting a column (field) from a line.

Syntax: `cut -d <separator> -f <field_number>`

- `-d ','` — comma is the **delimiter** (separator)
- `-f 1`   — take the 1st column (it starts from 1, not 0)

## Step 2
Example:
```bash
echo "ali,25,toshkent" | cut -d ',' -f 1
# ali

echo "ali,25,toshkent" | cut -d ',' -f 2
# 25

echo "ali,25,toshkent" | cut -d ',' -f 1,3
# ali,toshkent
```

## Step 3
It also works with a file:
```bash
cut -d',' -f1 users.csv
```
