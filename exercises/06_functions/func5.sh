#!/usr/bin/env bash
#
# MASHQ: Funksiyalar kompozitsiyasi
# DARAJA: ★★★★☆
# MAVZU: part2/01-functions · funksiyalar bir-birini chaqirish
#
# Ikkita funksiya yozing:
#   - `add a b`      — yig'indi
#   - `multiply a b` — ko'paytma
#
# Asosiy kodda: 3 va 4 ning yig'indisini olib, 2 ga ko'paytiring.
# Natija: (3 + 4) × 2 = 14
#
# Maslahat:
#   - Har funksiya `echo` orqali natija qaytarsin
#   - Asosiy kodda command substitution `$()` bilan chaqiring
#   - Misol: `r=$(add 1 2)` keyin `r2=$(multiply "$r" 3)`

# I AM NOT DONE

# TODO: add funksiyasi


# TODO: multiply funksiyasi


# Asosiy kod
sum=$(add 3 4)
result=$(multiply "$sum" 2)
echo "$result"

# === TEST META ===
# @test:stdout: 14
# @test:exit: 0
