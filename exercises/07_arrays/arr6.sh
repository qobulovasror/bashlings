#!/usr/bin/env bash
#
# MASHQ: Stringni massivga ajratish
# DARAJA: ★★★★☆
# MAVZU: part2/02-arrays · IFS + read -ra
#
# `csv` o'zgaruvchisida vergul bilan ajratilgan 3 ta ism bor.
# Uni `names` massiviga ajrating va 2-elementni chiqaring (Vali).
#
# Maslahat:
#   - `IFS=',' read -ra names <<< "$csv"`
#       -r — backslash escape qilmaslik
#       -a — natijani massivga yozish
#       <<< — here-string (bitta qator stdin)
#   - `IFS=','` faqat shu read uchun amal qiladi (qulay!)
#   - 2-element = `${names[1]}` (0'dan boshlanadi)

# I AM NOT DONE

csv="Ali,Vali,Gulnora"

# TODO: csv'ni names massivga ajrating


# TODO: 2-elementni chiqaring


# === TEST META ===
# @test:stdout: Vali
# @test:exit: 0
