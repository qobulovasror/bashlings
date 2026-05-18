#!/usr/bin/env bash
#
# MASHQ: Default argument
# DARAJA: ★★★☆☆
# MAVZU: part2/01-functions · ${1:-default}
#
# `greet` funksiyasi argument berilmasa "Anonim" deb chiqarsin.
# Argument berilsa — uni ishlatsin.
#
# Kutilgan natija (ikki marta chaqirilganda):
#     Salom, Ali!
#     Salom, Anonim!
#
# Maslahat:
#   - `${1:-default}` — agar $1 bo'sh yoki yo'q bo'lsa, "default" qaytaradi
#   - Funksiya ichida `local name="${1:-Anonim}"`

# I AM NOT DONE

greet() {
    # TODO: ${1:-Anonim} orqali default qiymat bilan name o'zgaruvchisi
    local name="$1"
    echo "Salom, $name!"
}

# Asosiy kod
greet "Ali"
greet

# === TEST META ===
# @test:stdout-cmd: printf 'Salom, Ali!\nSalom, Anonim!\n'
# @test:exit: 0
