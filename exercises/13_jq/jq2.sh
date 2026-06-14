#!/usr/bin/env bash
#
# MASHQ: Array iteratsiya
# DARAJA: ★★★☆☆
# MAVZU: part3/03-jq · .[].field
#
# JSON massivdan har user'ning `name` ini alohida qatorda chiqaring:
#     Ali
#     Vali
#     Gulnora
#
# Maslahat:
#   - `.[]` — array elementlarini iteratsiya qiladi
#   - `.[].name` — har element'ning name'ini
#
# --- English ---
# TASK: Array iteration
# LEVEL: ★★★☆☆
# TOPIC: part3/03-jq · .[].field
#
# From the JSON array, print each user's `name` on its own line:
#     Ali
#     Vali
#     Gulnora
#
# Hint:
#   - `.[]` — iterates over the array elements
#   - `.[].name` — the name of each element

# I AM NOT DONE

input='[{"name":"Ali"},{"name":"Vali"},{"name":"Gulnora"}]'

# TODO: har user.name ni alohida qatorda chiqaring
echo "$input" | jq '.'

# === TEST META ===
# @test:stdout-cmd: printf 'Ali\nVali\nGulnora\n'
# @test:exit: 0
