# 💡 sed1

## 1-bosqich
`sed 's/A/B/'` — birinchi `A` ni `B` ga almashtiradi har qatorda.

## 2-bosqich
Har qatorda faqat bir marta `old` bor, shuning uchun `g` shart emas. Lekin xavfsiz har doim `g` ishlatish ham mumkin.

## ✅ Yechim
```bash
sed 's/old/new/' input.txt
```

Yoki global:
```bash
sed 's/old/new/g' input.txt
```
