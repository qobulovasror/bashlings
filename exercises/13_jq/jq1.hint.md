# 💡 jq1

## 1-bosqich
`jq` sintaksisi: `jq '.field'` — object'dan maydonni olish.

## 2-bosqich
Default'da `jq` qo'shtirnoq bilan chiqaradi:
```bash
echo '{"name":"Ali"}' | jq '.name'      # "Ali"
echo '{"name":"Ali"}' | jq -r '.name'   # Ali — raw
```

Shell o'zgaruvchisida yoki test uchun har doim `-r` ishlating.
