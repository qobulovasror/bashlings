#!/usr/bin/env bash
#
# MASHQ: Faylning boshini ko'rish
# DARAJA: ★★☆☆☆
# MAVZU: part1/04-text-processing · head
#
# log.txt da 10 ta qator (1 dan 10 gacha sonlar).
# Faqat DASTLABKI 3 ta qatorni chiqaring.
#
# Maslahat: `head -n <son> <fayl>`
#
# --- English ---
# TASK: View the beginning of a file
# LEVEL: ★★☆☆☆
# TOPIC: part1/04-text-processing · head
#
# log.txt has 10 lines (numbers 1 to 10).
# Print only the FIRST 3 lines.
#
# Hint: `head -n <number> <file>`

# I AM NOT DONE

work=/tmp/bashlings-text1
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
seq 1 10 > log.txt

# TODO: log.txt'ning birinchi 3 qatorini chiqaring

# === TEST META ===
# @test:stdout-cmd: printf '1\n2\n3\n'
# @test:exit: 0
