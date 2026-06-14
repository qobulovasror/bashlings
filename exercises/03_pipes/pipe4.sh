#!/usr/bin/env bash
#
# MASHQ: Oxirgi N elementlarni olish
# DARAJA: ★★☆☆☆
# MAVZU: part1/03-pipes-redirection · | + tail
#
# 1 dan 10 gacha sonlarning OXIRGI 3 TASINI chiqaring:
#     8
#     9
#     10
#
# Maslahat:
#   - `seq 1 10`     — 1 dan 10 gacha har qatorda bitta son chiqaradi
#   - `tail -n 3`    — input'ning oxirgi 3 qatorini chiqaradi
#   - bularni `|` bilan bog'lang
#
# --- English ---
# TASK: Take the last N items
# LEVEL: ★★☆☆☆
# TOPIC: part1/03-pipes-redirection · | + tail
#
# Print the LAST 3 of the numbers from 1 to 10:
#     8
#     9
#     10
#
# Hint:
#   - `seq 1 10`     — prints numbers 1 to 10, one per line
#   - `tail -n 3`    — prints the last 3 lines of the input
#   - connect them with `|`

# I AM NOT DONE

# TODO: seq + pipe + tail

# === TEST META ===
# @test:stdout-cmd: printf '8\n9\n10\n'
# @test:exit: 0
