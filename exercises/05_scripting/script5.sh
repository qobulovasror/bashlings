#!/usr/bin/env bash
#
# MASHQ: case operatori
# DARAJA: ★★★★☆
# MAVZU: part1/05-basic-scripting · case
#
# `tugma` o'zgaruvchisi "b" qiymatiga ega.
# `case` ishlatib quyidagi mantiqni yozing:
#   "a" → "Birinchi"
#   "b" → "Ikkinchi"
#   "c" → "Uchinchi"
#   boshqa har qanday qiymat → "Noma'lum"
#
# Sintaksis:
#     case "$variable" in
#         pattern1)
#             commands
#             ;;
#         pattern2|pattern3)   # bir necha shart birlashtirilishi mumkin
#             commands
#             ;;
#         *)                    # default — har doim oxirida
#             commands
#             ;;
#     esac

# I AM NOT DONE

tugma="b"

# TODO: case bilan to'g'ri tarmoqni tanlang

# === TEST META ===
# @test:stdout: Ikkinchi
# @test:exit: 0
