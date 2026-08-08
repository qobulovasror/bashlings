# 💡 script7 — Maslahatlar

## 1-bosqich
Skriptga berilgan argumentlar maxsus o'zgaruvchilarda turadi:

| O'zgaruvchi | Ma'nosi                       |
|-------------|-------------------------------|
| `$0`        | skript nomi                   |
| `$1`, `$2`  | birinchi, ikkinchi argument   |
| `$#`        | argumentlar soni              |
| `"$@"`      | barcha argumentlar, alohida   |

```bash
echo "Jami: $#"
echo "Birinchi: $1"
```

## 2-bosqich
`"$@"` va `$@` orasidagi farq — bu bobning eng muhim jumbog'i.

`Vali Aliyev` bitta argument, lekin ichida bo'shliq bor:

```bash
printf '%s\n' $@      # 3 ta qator: Ali / Vali / Aliyev  ← noto'g'ri
printf '%s\n' "$@"    # 2 ta qator: Ali / Vali Aliyev    ← to'g'ri
```

Tirnoqsiz `$@` avval bitta satrga aylanadi, keyin bo'shliq bo'yicha
bo'linadi (word splitting). Tirnoq bu bo'linishni to'xtatadi.

## 3-bosqich
`printf` format satrini argumentlar tugaguncha qayta ishlatadi, shuning uchun
ikkita `%s` ikkita argumentni bitta qatorga joylaydi:

```bash
printf 'Barchasi: %s | %s\n' "$@"
```
