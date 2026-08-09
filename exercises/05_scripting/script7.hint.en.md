# 💡 script7 — Hints

## Step 1
The arguments given to a script live in special variables:

| Variable    | Meaning                        |
|-------------|--------------------------------|
| `$0`        | script name                    |
| `$1`, `$2`  | first, second argument         |
| `$#`        | number of arguments            |
| `"$@"`      | all arguments, kept separate   |

```bash
echo "Jami: $#"
echo "Birinchi: $1"
```

## Step 2
The difference between `"$@"` and `$@` is this chapter's key puzzle.

`Vali Aliyev` is one argument, but it contains a space:

```bash
printf '%s\n' $@      # 3 lines: Ali / Vali / Aliyev  ← wrong
printf '%s\n' "$@"    # 2 lines: Ali / Vali Aliyev    ← right
```

Unquoted, `$@` first becomes a single string and is then split on spaces
(word splitting). The quotes stop that split.

## Step 3
`printf` reuses its format string until the arguments run out, so two `%s`
placeholders put two arguments on one line:

```bash
printf 'Barchasi: %s | %s\n' "$@"
```
