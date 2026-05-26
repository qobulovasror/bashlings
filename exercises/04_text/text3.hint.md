# 💡 text3 — Maslahatlar

## 1-bosqich
**Eng muhim tushuncha:** sukut bo'yicha `sort` matn ko'rinishida tartiblaydi (lexicographic):
```bash
echo $'100\n23\n5' | sort
# 100
# 23
# 5     ← noto'g'ri tartib raqamlar uchun!
```

Sababi: `'1'` < `'2'` < `'5'` (birinchi belgi bo'yicha taqqoslaydi).

## 2-bosqich
`-n` flagi — **numeric sort**. Raqamlarni qiymati bo'yicha tartiblaydi:
```bash
echo $'100\n23\n5' | sort -n
# 5
# 23
# 100
```
