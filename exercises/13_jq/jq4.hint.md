# 💡 jq4

## 1-bosqich
Yangi object yaratish — fatigure brackets:
```bash
echo '{"x":1}' | jq '{y: .x}'
# {"y": 1}
```

## 2-bosqich
String concatenation `+` bilan:
```bash
echo '{"a":"hi","b":"ali"}' | jq '{msg: (.a + " " + .b)}'
# {"msg": "hi ali"}
```

⚠ Qavslarni unutmang: `{key: (.x + .y)}` — aks holda yoki sintaksis xato yoki noto'g'ri parse bo'ladi.

## 3-bosqich
`-c` (compact) — pretty-print emas, bir qator JSON:
```bash
jq '{a: 1}'    # multi-line
jq -c '{a: 1}' # {"a":1}
```
