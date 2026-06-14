# 💡 ssh5

## Step 1
rsync syntax:
```
rsync <FLAGS> <SOURCE> <USER>@<HOST>:<DEST>
```

## Step 2
The most classic flag combination: `-avz --delete`

## Step 3
The `/` difference matters:
- `./dist`   — copies the folder
- `./dist/`  — copies the files inside it

In this exercise we use `/dist/` (the contents).
