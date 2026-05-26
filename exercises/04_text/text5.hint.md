# 💡 text5 — Maslahatlar

## 1-bosqich
"5-7 qatorlarni olish" ni ikki bosqichda hal qilamiz:
1. Birinchi **7** qatorini olish — `head -n 7`
2. Yo'natilgan natijaning **oxirgi 3** qatori (5, 6, 7) — `tail -n 3`

## 2-bosqich
Ikkalasini `|` (pipe) bilan zanjirlang:
```bash
head -n 7 big.txt | tail -n 3
```

Aqliy model:
```
big.txt: 1 2 3 4 5 6 7 8 9 ... 100
   ↓ head -n 7
         1 2 3 4 5 6 7
                  ↓ tail -n 3
                       5 6 7
```

## 3-bosqich
**Bonus** — bu pattern juda ko'p ishlatiladi:
- 50-100 qatorlar: `head -n 100 fayl | tail -n 51`
- Yoki `sed` bilan: `sed -n '5,7p' big.txt` (sed mavzusi 2-qismda)
