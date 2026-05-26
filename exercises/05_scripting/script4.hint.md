# 💡 script4 — Maslahatlar

## 1-bosqich
Bashda `for` loopning 3 ta asosiy shakli:
```bash
# A. Ro'yxat bo'yicha
for x in 1 2 3 4 5; do echo "$x"; done

# B. Diapazon (brace expansion)
for x in {1..5}; do echo "$x"; done

# C. C-style
for ((i=1; i<=5; i++)); do echo "$i"; done
```

## 2-bosqich
Bu mashqda eng qisqasi — **B variant**.

## 3-bosqich
`do ... done` bloki ichida bajariladigan kod yoziladi.
