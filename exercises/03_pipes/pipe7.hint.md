# 💡 pipe7 — Maslahatlar

## 1-bosqich
Oddiy `>` redirect natijani **faqat faylga** yo'naltiradi — ekranda hech narsa ko'rinmaydi:
```bash
echo "salom" > out.txt   # ekran bo'sh, fayl to'la
```

## 2-bosqich
`tee` — stdin'dan o'qiydi va ikkala tomonga (stdout VA fayl) yozadi:
```bash
echo "salom" | tee out.txt
```

## 3-bosqich
`tee -a fayl.txt` — append rejimi (`>>` ekvivalenti).
