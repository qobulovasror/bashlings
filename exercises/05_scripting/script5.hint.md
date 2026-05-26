# 💡 script5 — Maslahatlar

## 1-bosqich
`case` — `if/elif/elif/else` ning chiroyli alternativasi. Bir o'zgaruvchini ko'p qiymatga taqqoslashda ravonroq.

## 2-bosqich
Sintaksis:
```bash
case "$variable" in
    "a")
        echo "Birinchi"
        ;;
    "b")
        echo "Ikkinchi"
        ;;
    *)
        echo "Boshqa"
        ;;
esac
```

Diqqat:
- Har tarmoq oxirida `;;` (ikkita nuqtali vergul)
- `*)` — default tarmoq, oxirida turadi
- `esac` — `case` ni teskari yozilgan (yopuvchi belgi)

## 3-bosqich
`tugma` = "b" → "Ikkinchi" tarmog'iga tushadi.
