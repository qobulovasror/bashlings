# 💡 text2 — Maslahatlar

## 1-bosqich
`grep -i` — **case-insensitive** rejim. `error`, `Error`, `ERROR` — uchchalasini ham topadi.

```bash
grep -i "error" log.txt   # 3 ta qator topadi
```

## 2-bosqich
`grep -c` — moslashlar **sonini** (soxir matn emas) chiqaradi:
```bash
grep -c "error" log.txt   # 3
```

## 3-bosqich
Ikki flagni birlashtirib ishlatish mumkin:
```bash
grep -ic "error" log.txt
```
