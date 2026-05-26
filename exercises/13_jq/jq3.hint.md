# 💡 jq3

## 1-bosqich
`select(condition)` — shartga mos kelganlarini qoldiradi:
```bash
echo '[{"a":1},{"a":2},{"a":3}]' | jq '.[] | select(.a > 1)'
# {"a":2}
# {"a":3}
```

## 2-bosqich
`length` — array uzunligi. Lekin `.[]` ko'p output qaytaradi, `length` ishlamaydi. **Array'ga o'rab oling**:
```bash
echo '[1,2,3]' | jq '[.[] | select(. > 1)] | length'
# 2
```

## 3-bosqich
Yoki `map` orqali — ekvivalent:
```bash
echo '[1,2,3]' | jq 'map(select(. > 1)) | length'
```
