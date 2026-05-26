# 💡 net5

## 1-bosqich
Multi-line string'da PATTERN'ga mos qatorlarni topish:
```bash
echo "$scan_output" | grep "succeeded"
```

## 2-bosqich
Faqat SONINI olish uchun `-c` flag — qatorlarni emas, **sonni** chiqaradi:
```bash
echo "$scan_output" | grep -c "succeeded"
```

## 3-bosqich
Diqqat: `wc -l` butun string qatorlari sonini sanaydi (5 ta) — bizga
faqat OPEN portlar kerak, shuning uchun `grep -c` to'g'ri.
