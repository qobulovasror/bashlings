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
