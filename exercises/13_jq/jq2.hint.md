# 💡 jq2

## 1-bosqich
`.[]` — massiv elementlarini **alohida-alohida** chiqaradi (multi-output).

```bash
echo '[1,2,3]' | jq '.[]'
# 1
# 2
# 3
```

## 2-bosqich
Har element'ning maydoniga `.field` bilan kirish:
```bash
echo '[{"a":1},{"a":2}]' | jq '.[].a'
# 1
# 2
```

## 3-bosqich
`-r` har doim, qo'shtirnoqlar olib tashlash uchun.
