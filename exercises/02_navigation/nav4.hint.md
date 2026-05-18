# 💡 nav4 — Maslahatlar

## 1-bosqich
**Brace expansion** — shell `{a,b,c}` ni avtomatik `a b c` ga ochib beradi (probel bilan ajratiladi).

## 2-bosqich
Misollar:
- `echo {1,2,3}` → `1 2 3`
- `touch file{1,2,3}.txt` → file1.txt, file2.txt, file3.txt yaratadi

## 3-bosqich
Diapazon ham mumkin: `{1..3}` xuddi `{1,2,3}` kabi ishlaydi.

## ✅ Yechim
```bash
touch file{1,2,3}.txt
```

Yoki:
```bash
touch file{1..3}.txt
```
