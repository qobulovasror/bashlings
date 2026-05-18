# 07 — Massivlar va lug'atlar

📘 **Kitob bobi:** [`docs/part2/02-arrays.md`](../../docs/part2/02-arrays.md)

## Mashqlar ro'yxati

| # | Nomi    | Mavzu                                | Daraja      |
|---|---------|--------------------------------------|-------------|
| 1 | `arr1`  | E'lon qilish + `"${arr[@]}"`         | ★★☆☆☆       |
| 2 | `arr2`  | `arr+=(...)` qo'shish + oxirgi element| ★★★☆☆      |
| 3 | `arr3`  | Element soni — `${#arr[@]}`          | ★★★☆☆       |
| 4 | `arr4`  | Iteratsiya indekslar bilan           | ★★★☆☆       |
| 5 | `arr5`  | **Associative array** (`declare -A`) | ★★★★☆       |
| 6 | `arr6`  | Stringni massivga (`IFS` + `read`)   | ★★★★☆       |

::: warning macOS diqqat
`arr5` — `declare -A` Bash 4+ talab qiladi. macOS stock bash 3.2 ishlamaydi. `brew install bash` qiling.

`arr2` — `${arr[-1]}` Bash 4.2+ talab qiladi.

## Boshlash

```bash
bashlings watch
```
