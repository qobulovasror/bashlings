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
#
# --- English ---
# TASK: Split a string into an array
# LEVEL: ★★★★☆
# TOPIC: part2/02-arrays · IFS + read -ra
#
# The `csv` variable holds 3 names separated by commas.
# Split it into a `names` array and print the 2nd element (Vali).
#
# Hint:
#   - `IFS=',' read -ra names <<< "$csv"`
#       -r — do not escape backslashes
#       -a — write the result into an array
#       <<< — here-string (one line of stdin)
#   - `IFS=','` applies only to this read (handy!)
#   - 2nd element = `${names[1]}` (starts from 0)

# I AM NOT DONE

csv="Ali,Vali,Gulnora"

# TODO: csv'ni names massivga ajrating


# TODO: 2-elementni chiqaring


# === TEST META ===
# @test:stdout: Vali
# @test:exit: 0
