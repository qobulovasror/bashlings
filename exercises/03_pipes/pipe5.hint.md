# 💡 pipe5 — Maslahatlar

## 1-bosqich
`grep` — qatorlar ichida pattern qidiradi:
```bash
grep "ERROR" log.txt   # ERROR satrlarini chiqaradi
```

## 2-bosqich
`-c` flagi — moslashlar sonini **toza son** sifatida chiqaradi (qatorlarni emas).

```bash
grep -c "ERROR" log.txt
# 2
```

## 3-bosqich
Bu eng tezkor usul. Pipe ham mumkin, lekin `wc -l` macOS'da leading whitespace beradi:
```bash
grep "ERROR" log.txt | wc -l   # "       2" — bo'shliqlar bilan
```
