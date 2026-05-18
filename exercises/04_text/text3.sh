#!/usr/bin/env bash
#
# MASHQ: Son bo'yicha tartiblash
# DARAJA: ★★★☆☆
# MAVZU: part1/04-text-processing · sort -n
#
# scores.txt da 5 ta son tartibsiz yozilgan: 42, 5, 100, 7, 23
# Ularni RAQAM bo'yicha o'sish tartibida chiqaring.
#
# ⚠ Diqqat: oddiy `sort` LEXICOGRAPHIC tartiblaydi!
#     sort: 100, 23, 42, 5, 7   ← noto'g'ri
#     sort -n: 5, 7, 23, 42, 100 ← to'g'ri (raqam bo'yicha)
#
# Maslahat: `-n` flagi (numeric).

# I AM NOT DONE

work=/tmp/bashlings-text3
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf '42\n5\n100\n7\n23\n' > scores.txt

# TODO: scores.txt'ni raqam bo'yicha tartiblang

# === TEST META ===
# @test:stdout-cmd: printf '5\n7\n23\n42\n100\n'
# @test:exit: 0
