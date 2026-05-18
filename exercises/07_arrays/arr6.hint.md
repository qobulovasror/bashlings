# 💡 arr6

## 1-bosqich
`read -ra` — stringni massivga ajratish:
- `-r` — backslash escape qilmaslik (har doim ishlating)
- `-a names` — natijani `names` massiviga yozish

## 2-bosqich
`IFS=','` — vergul bo'yicha ajratish:
```bash
IFS=',' read -ra names <<< "$csv"
```

`<<<` — here-string (bitta qatorli stdin).

`IFS=','` faqat shu `read` uchun amal qiladi — global'ga ta'sir qilmaydi.

## ✅ Yechim
```bash
IFS=',' read -ra names <<< "$csv"
echo "${names[1]}"
```
