# 💡 cron6

## 1-bosqich
`cut` bilan maydonlar ajratish:
```bash
echo "a b c d e f g" | cut -d' ' -f6
# f
```

- `-d' '` — delimiter bo'sh joy
- `-f6`   — 6-maydon

## 2-bosqich
6-MAYDONDAN OXIRGACHA hammasi:
```bash
echo "a b c d e f g" | cut -d' ' -f6-
# f g
```

`-f6-` — "6 dan oxirigacha".

## 3-bosqich
Bizning qatorda 5 ta vaqt maydoni + buyruq:
```bash
echo "$line" | cut -d' ' -f6-
```
