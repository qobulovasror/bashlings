#!/usr/bin/env bash
#
# MASHQ: local — scope izolyatsiya
# DARAJA: ★★★☆☆
# MAVZU: part2/01-functions · local kalit so'zi
#
# Quyidagi skriptda BUG bor: `inner` funksiyasi `name` o'zgaruvchisini
# tashqi scope'ga "sizdiradi". `local` qo'shib tuzating.
#
# Kutilgan natija:
#     Funksiya ichida: ichki
#     Funksiya tashqarisida: tashqi
#
# (Hozir oxirgi qator "Funksiya tashqarisida: ichki" bo'lib chiqadi — BUG)
#
# Maslahat:
#   - Funksiya ichida har yangi o'zgaruvchi uchun `local` ishlating
#
# --- English ---
# TASK: local — scope isolation
# LEVEL: ★★★☆☆
# TOPIC: part2/01-functions · the local keyword
#
# The script below has a BUG: the `inner` function "leaks" the `name`
# variable into the outer scope. Fix it by adding `local`.
#
# Expected result:
#     Funksiya ichida: ichki
#     Funksiya tashqarisida: tashqi
#
# (Right now the last line comes out as "Funksiya tashqarisida: ichki" — the BUG)
#
# Hint:
#   - Use `local` for every new variable inside a function

# I AM NOT DONE

inner() {
    # TODO: `local` qo'shing
    name="ichki"
    echo "Funksiya ichida: $name"
}

name="tashqi"
inner
echo "Funksiya tashqarisida: $name"

# === TEST META ===
# @test:stdout-cmd: printf 'Funksiya ichida: ichki\nFunksiya tashqarisida: tashqi\n'
# @test:exit: 0
