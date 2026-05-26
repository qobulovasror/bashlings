# 💡 jq6

## 1-bosqich
`add` — array elementlari yig'indisini hisoblaydi:
```bash
echo '[1,2,3,4]' | jq 'add'
# 10
```

Lekin `add` array kutadi — object massivga to'g'ridan-to'g'ri qo'llab bo'lmaydi.

## 2-bosqich
Avval narxlar massivini ajratib oling:
```bash
echo '[{"p":1},{"p":2}]' | jq '[.[].p]'
# [1, 2]
```

Keyin yig'indi:
```bash
echo '[{"p":1},{"p":2}]' | jq '[.[].p] | add'
# 3
```

## 3-bosqich
`map` bilan ham ekvivalent:
```bash
jq 'map(.price) | add'
```
