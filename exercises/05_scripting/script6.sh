#!/usr/bin/env bash
#
# MASHQ: while loop bilan hisoblagich
# DARAJA: ★★★★☆
# MAVZU: part1/05-basic-scripting · while + counter
#
# Hisoblagichni `i=1` dan boshlang. while shart `i <= 3` bo'lguncha aylantirin.
# Har qadamda chiqaring:
#     Qadam 1
#     Qadam 2
#     Qadam 3
#
# Maslahat:
#   - while sintaksis:
#         while [[ shart ]]; do
#             ...
#         done
#   - Hisoblagichni oshirish: `((i++))` yoki `i=$((i + 1))`
#
# --- English ---
# TASK: a counter with a while loop
# LEVEL: ★★★★☆
# TOPIC: part1/05-basic-scripting · while + counter
#
# Start the counter at `i=1`. Loop while the condition `i <= 3` holds.
# On each step print:
#     Qadam 1
#     Qadam 2
#     Qadam 3
#
# Hint:
#   - while syntax:
#         while [[ condition ]]; do
#             ...
#         done
#   - Increment the counter: `((i++))` or `i=$((i + 1))`

# I AM NOT DONE

# TODO: i=1 dan boshlang, while loop yozing

# === TEST META ===
# @test:stdout-cmd: printf 'Qadam 1\nQadam 2\nQadam 3\n'
# @test:exit: 0
