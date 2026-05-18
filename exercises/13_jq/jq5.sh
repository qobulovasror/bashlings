#!/usr/bin/env bash
#
# MASHQ: Eng yuqori ball egasi
# DARAJA: ★★★★☆
# MAVZU: part3/03-jq · max_by(...)
#
# Talabalar ro'yxatidan ENG YUQORI ball egasining ismini chiqaring.
#
# Input:    3 ta talaba — A(80), B(95), C(70)
# Kutilgan: B
#
# Maslahat:
#   - `max_by(.score)` — eng katta score'li elementni qaytaradi
#   - Keyin uning `.name`'ini chiqaring
#   - `-r` — qo'shtirnoqsiz

# I AM NOT DONE

input='[{"name":"A","score":80},{"name":"B","score":95},{"name":"C","score":70}]'

# TODO: eng yuqori ball egasi ismini chiqaring (raw)
echo "$input" | jq '.'

# === TEST META ===
# @test:stdout: B
# @test:exit: 0
