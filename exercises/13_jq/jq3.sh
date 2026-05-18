#!/usr/bin/env bash
#
# MASHQ: Filter + sanash
# DARAJA: ★★★★☆
# MAVZU: part3/03-jq · select() + length
#
# Yoshlar ro'yxatidan KATTALAR (yoshi >= 18) SONINI chiqaring.
#
# Input: 5 ta odam — 15, 20, 17, 30, 22 yoshda
# Kutilgan: 3   (20, 30, 22 — kattalar)
#
# Maslahat:
#   - `.[] | select(.age >= 18)` — filtering
#   - `[ ... | select(...) ] | length` — sanash uchun array'ga o'rab, keyin length
#   - Yoki: `map(select(...)) | length`

# I AM NOT DONE

input='[{"age":15},{"age":20},{"age":17},{"age":30},{"age":22}]'

# TODO: kattalar (yoshi >= 18) sonini chiqaring
echo "$input" | jq '.'

# === TEST META ===
# @test:stdout: 3
# @test:exit: 0
