# 💡 jq4

## Step 1
Create a new object — curly brackets:
```bash
echo '{"x":1}' | jq '{y: .x}'
# {"y": 1}
```

## Step 2
String concatenation with `+`:
```bash
echo '{"a":"hi","b":"ali"}' | jq '{msg: (.a + " " + .b)}'
# {"msg": "hi ali"}
```

⚠ Don't forget the parentheses: `{key: (.x + .y)}` — otherwise you get either a syntax error or an incorrect parse.

## Step 3
`-c` (compact) — not pretty-print, JSON on a single line:
```bash
jq '{a: 1}'    # multi-line
jq -c '{a: 1}' # {"a":1}
```
