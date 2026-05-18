# 💡 pipe6 — Maslahatlar

## 1-bosqich
`sort` — qatorlarni alfabet bo'yicha tartiblaydi.

## 2-bosqich
`uniq` — **ketma-ket** takrorlarni o'chiradi. Shuning uchun har doim `sort | uniq` tartibida ishlatiladi.

```bash
cat fruits.txt | sort | uniq
```

Yoki qisqaroq (faylni argumentga olish):
```bash
sort fruits.txt | uniq
```

## 3-bosqich
**Bonus:** `sort -u` aynan shu vazifani bitta buyruqda bajaradi.

## ✅ Yechim
```bash
sort fruits.txt | uniq
```

Yoki:
```bash
sort -u fruits.txt
```
