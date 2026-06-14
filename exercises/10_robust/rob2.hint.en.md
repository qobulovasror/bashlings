# 💡 rob2

## Step 1
**Word-splitting trap**: if `name="Ali Vali"`, then unquoted `$name` is 2 words:
```bash
ls $name        # ls Ali Vali — two arguments!
```

## Step 2
Inside double quotes — a single argument:
```bash
ls "$name"      # ls "Ali Vali" — one argument
```

## Step 3
**Rule:** always write it as `"$var"`. ShellCheck SC2086 exists exactly for this.
