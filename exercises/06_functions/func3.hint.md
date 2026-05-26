# 💡 func3 — Maslahatlar

## 1-bosqich
**Parameter expansion** `${var:-default}`:
- Agar `var` bo'sh yoki aniqlanmagan — `default` qaytaradi
- Aks holda — `var` qiymatini

```bash
echo "${1:-Anonim}"
# $1 berilgan: $1 qiymati
# $1 berilmagan: Anonim
```

## 2-bosqich
Bu sintaksisni `local` bilan birlashtiring:
```bash
greet() {
    local name="${1:-Anonim}"
    echo "Salom, $name!"
}
```

## 3-bosqich
**O'xshash variantlar:**
- `${var:-default}` — default qiymat (var o'zgarmaydi)
- `${var:=default}` — default va var ga ham o'rnatadi
- `${var:?xato}` — bo'sh bo'lsa xato bilan chiqadi
- `${var:+almashtir}` — bo'sh emas bo'lsa, almashtirilgan
