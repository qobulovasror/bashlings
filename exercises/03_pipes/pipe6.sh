#!/usr/bin/env bash
#
# MASHQ: Takrorlarsiz tartibli ro'yxat
# DARAJA: ★★★☆☆
# MAVZU: part1/03-pipes-redirection · sort | uniq
#
# fruits.txt fayli ichida bir nechta meva nomi takror bilan yozilgan.
# Ularni TAKRORSIZ va ALFAVIT BO'YICHA chiqaring:
#     banan
#     olma
#     uzum
#
# Maslahat:
#   - `sort <fayl>`   — qatorlarni alfabet bo'yicha tartiblaydi
#   - `uniq`          — KETMA-KET takrorlarni o'chiradi (sort'dan keyin ishlatish)
#   - Ularni `|` bilan bog'lang

# I AM NOT DONE

work=/tmp/bashlings-pipe6
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf 'olma\nuzum\nolma\nbanan\nuzum\n' > fruits.txt

# TODO: sort + uniq pipeline

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' banan olma uzum
# @test:exit: 0
