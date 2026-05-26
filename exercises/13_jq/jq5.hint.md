# 💡 jq5

## 1-bosqich
`max_by(filter)` — array'dan eng katta element'ni qaytaradi (`filter`'ga ko'ra):
```bash
echo '[{"x":3},{"x":1},{"x":5}]' | jq 'max_by(.x)'
# {"x": 5}
```

`min_by` ham bor — eng kichik.

## 2-bosqich
Olingan object'dan kerakli maydonni ajratish — `|` orqali:
```bash
echo '[{"x":3,"n":"A"},{"x":5,"n":"B"}]' | jq 'max_by(.x) | .n'
# "B"
```

## 3-bosqich
`-r` har doim — raw output.
