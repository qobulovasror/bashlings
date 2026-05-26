# 💡 script6 — Maslahatlar

## 1-bosqich
`while` — shart **TRUE** bo'lguncha takrorlaydi:
```bash
i=1
while [[ $i -le 3 ]]; do
    echo "Qadam $i"
    ((i++))
done
```

## 2-bosqich
`((i++))` — arifmetik kontekst — `i = i + 1` ga teng (qisqaroq).

Muqobillari:
- `i=$((i + 1))`
- `let i=i+1`
- `((i += 1))`

## 3-bosqich
⚠ Hisoblagichni oshirishni unutmang — aks holda **cheksiz loop** bo'ladi!
