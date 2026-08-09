# 💡 script8 — Maslahatlar

## 1-bosqich
`read` stdin'dan **bitta qator** o'qib, o'zgaruvchiga yozadi:

```bash
read -r name
echo "Salom, $name!"
```

`-r` teskari chiziqni oddiy belgi deb qabul qiladi. Deyarli har doim kerak —
`-r` siz `C:\temp` kabi matn buziladi.

## 2-bosqich
`read` faqat bitta qatorni "yeydi" — qolgan qatorlar stdin'da turaveradi.
Ya'ni birinchi `read` dan keyin oqim tugamaydi.

## 3-bosqich
Qolganini sanash — `while read` sikli oqim tugaguncha aylanadi:

```bash
qolgan=0
while read -r _; do
    qolgan=$((qolgan + 1))
done
echo "Qolganlari: $qolgan"
```

`_` — "qiymati kerak emas" degan an'anaviy nom.
