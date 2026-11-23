#!/usr/bin/env bash
#
# MASHQ: awk bilan field tanlash
# DARAJA: ★★★☆☆
# MAVZU: part2/03-sed-awk · $N field
#
# data.txt har qatorida 3 ta probel bilan ajratilgan ustun:
#     ali 25 toshkent
#     vali 30 samarqand
#
# Faqat 3-ustun (shaharlar) ni chiqaring:
#     toshkent
#     samarqand
#
# --- English ---
# TASK: Selecting fields with awk
# LEVEL: ★★★☆☆
# TOPIC: part2/03-sed-awk · $N field
#
# Each line of data.txt has 3 columns separated by spaces:
#     ali 25 toshkent
#     vali 30 samarqand
#
# Print only the 3rd column (the cities):
#     toshkent
#     samarqand

printf 'ali 25 toshkent\nvali 30 samarqand\n' > data.txt

# TODO: awk bilan 3-ustunni chiqaring

# === TEST META ===
# @test:stdout-cmd: printf 'toshkent\nsamarqand\n'
# @test:exit: 0
