# 💡 rob4

## 1-bosqich
`mkdir` default:
- Yo'q bo'lsa — yaratadi (OK)
- Mavjud bo'lsa — **xato** (set -e bilan to'xtaydi)

## 2-bosqich
`mkdir -p`:
- Yo'q bo'lsa — yaratadi
- Mavjud bo'lsa — **OK** (xato yo'q)
- Yo'lda boshqa kataloglar yo'q bo'lsa — ham yaratadi

## 3-bosqich
**Idempotent skript** — bir necha marta ishga tushirsangiz ham xato bermaydi.
