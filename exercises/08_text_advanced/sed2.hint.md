# 💡 sed2

## 1-bosqich
`sed 'Nd'` — N-qatorni o'chiradi. Misol:
```bash
sed '1d' file       # 1-qator o'chadi
sed '5,10d' file    # 5-10 oraliq
```

## 2-bosqich
Bir nechta qatorni alohida o'chirish — `;` orqali:
```bash
sed '2d;4d' log.txt
```

Yoki `-e` bilan:
```bash
sed -e '2d' -e '4d' log.txt
```

## ✅ Yechim
```bash
sed '2d;4d' log.txt
```
