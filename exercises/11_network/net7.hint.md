# 💡 net7

## 1-bosqich
Massiv ustidan loop'ning klassik shabloni:
```bash
for x in "${ARR[@]}"; do
    echo "$x"
done
```

Bu yerda `"${ARR[@]}"` har elementni alohida arg sifatida beradi
(bo'sh joy bilan ajratish muammosi yo'q).

## 2-bosqich
Funksiya chiqishini olish — command substitution:
```bash
result=$(curl "$url")
echo "$result"   # masalan: 200
```

## 3-bosqich
Format `URL=STATUS`:
```bash
status=$(curl "$url")
echo "$url=$status"
```

Bularni for ichiga joylang.
