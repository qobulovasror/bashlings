# 02 — Fayl tizimi va navigatsiya

📘 **Kitob bobi:** [`docs/part1/02-navigation.md`](../../docs/part1/02-navigation.md)

## Mashqlar ro'yxati

| # | Nomi    | Mavzu                                  | Daraja      |
|---|---------|----------------------------------------|-------------|
| 1 | `nav1`  | `pwd` — joriy katalog                  | ★☆☆☆☆       |
| 2 | `nav2`  | `cd ~` — home katalogga                | ★☆☆☆☆       |
| 3 | `nav3`  | `mkdir -p` + bir nechta katalog        | ★★☆☆☆       |
| 4 | `nav4`  | `touch` + brace expansion `{1,2,3}`    | ★★☆☆☆       |
| 5 | `nav5`  | Wildcards — `ls *.txt`                 | ★★☆☆☆       |
| 6 | `nav6`  | `cp` + `cat` (mazmunni tekshirish)     | ★★☆☆☆       |
| 7 | `nav7`  | `mv` — fayl nomini o'zgartirish        | ★★☆☆☆       |
| 8 | `nav8`  | `rm` + qolganlarini ko'rsatish         | ★★★☆☆       |

## Sandbox

3–8 raqamli mashqlar **`/tmp/bashlings-navN/`** ichida ishlaydi — sizning workspace'ingiz ifloslanmaydi. Har skript boshida quyidagi shablon bor:

```bash
work=/tmp/bashlings-navN
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
```

## Boshlash

```bash
bashlings watch
```
