#!/usr/bin/env bash
#
# MASHQ: Iteratsiya indekslar bilan
# DARAJA: ★★★☆☆
# MAVZU: part2/02-arrays · "${!arr[@]}"
#
# `nums` massivida 3 ta son. Har element va uning POZITSIYASI (1'dan boshlab):
#     Element 1: 10
#     Element 2: 20
#     Element 3: 30
#
# Maslahat:
#   - `"${!arr[@]}"` — indekslar ro'yxati (0, 1, 2, ...)
#   - Pozitsiya = indeks + 1 (chunki 0'dan boshlanadi)
#   - Element: `${arr[$i]}`
#
# --- English ---
# TASK: Iterating with indexes
# LEVEL: ★★★☆☆
# TOPIC: part2/02-arrays · "${!arr[@]}"
#
# The `nums` array has 3 numbers. For each element print it and its POSITION (starting from 1):
#     Element 1: 10
#     Element 2: 20
#     Element 3: 30
#
# Hint:
#   - `"${!arr[@]}"` — the list of indexes (0, 1, 2, ...)
#   - Position = index + 1 (because it starts from 0)
#   - Element: `${arr[$i]}`

# I AM NOT DONE

nums=(10 20 30)

# TODO: for loop bilan indeks va element chiqaring


# === TEST META ===
# @test:stdout-cmd: printf 'Element 1: 10\nElement 2: 20\nElement 3: 30\n'
# @test:exit: 0
