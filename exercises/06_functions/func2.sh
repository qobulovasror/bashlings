#!/usr/bin/env bash
#
# MASHQ: Qiymat qaytaruvchi funksiya
# DARAJA: ★★★☆☆
# MAVZU: part2/01-functions · echo + command substitution
#
# `square` funksiyasini yozing. Argumentni qabul qiladi va
# uning kvadratini `echo` orqali qaytaradi.
#
# Asosiy kodda funksiyani 5 bilan chaqirib, natijani saqlang va
# quyidagi formatda chiqaring:
#     5 ning kvadrati: 25
#
# Maslahat:
#   - `return N` — faqat 0..255 exit code (qiymat emas!)
#   - Haqiqiy qiymat: `echo $((...))` + `result=$(square 5)` (command substitution)
#   - Arifmetika: `$((a * b))`

# I AM NOT DONE

# TODO: square funksiyasini yozing


# Asosiy kod (allaqachon tayyor — siz faqat funksiyani qo'shasiz)
result=$(square 5)
echo "5 ning kvadrati: $result"

# === TEST META ===
# @test:stdout: 5 ning kvadrati: 25
# @test:exit: 0
