# 💡 script3 — Maslahatlar

## 1-bosqich
Bashda `if` sintaksisi:
```bash
if [[ shart ]]; then
    # bajariladigan kod
else
    # default
fi
```

`;` — Linux uchun "yangi qator ekvivalenti". `then`'ni keyingi qatorga ham yozsa bo'ladi.

## 2-bosqich
**Sonli taqqoslash operatorlari:**
| Operator | Ma'no               |
|----------|---------------------|
| `-eq`    | teng (`==`)         |
| `-ne`    | teng emas (`!=`)    |
| `-gt`    | katta (`>`)         |
| `-lt`    | kichik (`<`)        |
| `-ge`    | katta yoki teng     |
| `-le`    | kichik yoki teng    |

⚠ `$yosh > 18` matn taqqoslash bo'lib qoladi — har doim `-gt` ishlating.

## 3-bosqich
Sizga kerak: `[[ $yosh -gt 18 ]]`
