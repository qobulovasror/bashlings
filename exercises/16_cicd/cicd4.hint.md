# 💡 cicd4

## 1-bosqich
Matrix — bir job'ni har xil parametrlar bilan parallel ishlatish:
```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest]
```

## 2-bosqich
`${{ matrix.os }}` — GitHub Actions runtime'da kengayadi (bash emas).
Bizning skript bunday matnni AYNAN shu ko'rinishda chiqarishi kerak.

Bash'da literal saqlash uchun:
- **bir tirnoq**: `'${{ matrix.os }}'`  → bash hech narsani o'zgartirmaydi
- yoki heredoc (default'da interpolatsiya bo'ladi, lekin `${{ }}` shaklini
  bash tushunmaydi va literal qoldiradi)

## 3-bosqich
Sinab ko'ring:
```bash
echo '${{ matrix.os }}'        # literal
```
