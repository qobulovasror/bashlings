# 💡 sed3

## 1-bosqich
**Capture groups**: regex ichidagi `(...)` — keyinchalik `\1`, `\2`, ... orqali murojaat qilinadi.

`-E` flagi extended regex'ni yoqadi (`(...)` literal emas):
```bash
sed -E 's/(.+)-(.+)/\2-\1/'
```

## 2-bosqich
Sana format: `YYYY-MM-DD`
- Yil: `([0-9]{4})` yoki `(....)` 
- Oy: `([0-9]{2})` yoki `(..)`
- Kun: `([0-9]{2})` yoki `(..)`

```bash
sed -E 's/([0-9]+)-([0-9]+)-([0-9]+)/\3\/\2\/\1/'
```

## 3-bosqich
`/` ni `\/` qilib escape qilish kerak (yoki ajratuvchini `|` qilish):
```bash
sed -E 's|(....)-(..)-(..)|\3/\2/\1|'
```

## ✅ Yechim
```bash
sed -E 's|([0-9]+)-([0-9]+)-([0-9]+)|\3/\2/\1|' date.txt
```

Yoki bosqichli:
```bash
sed -E 's/^(....)-(..)-(..)$/\3\/\2\/\1/' date.txt
```
