#!/usr/bin/env bash
#
# MASHQ: Field access
# DARAJA: ★★☆☆☆
# MAVZU: part3/03-jq · .field + -r
#
# Quyidagi JSON'dan FAQAT ism (`name`) ni chiqaring.
# Qo'shtirnoqsiz, sof matn ko'rinishida.
#
# Kutilgan:
#     Ali
#
# Maslahat:
#   - `jq '.name'`     — qo'shtirnoq bilan: "Ali"
#   - `jq -r '.name'`  — raw (qo'shtirnoqsiz): Ali
#
# --- English ---
# TASK: Field access
# LEVEL: ★★☆☆☆
# TOPIC: part3/03-jq · .field + -r
#
# From the JSON below, print ONLY the name (`name`).
# As plain text, without quotes.
#
# Expected:
#     Ali
#
# Hint:
#   - `jq '.name'`     — with quotes: "Ali"
#   - `jq -r '.name'`  — raw (without quotes): Ali

# I AM NOT DONE

input='{"name":"Ali","age":25,"city":"Toshkent"}'

# TODO: jq bilan name ni chiqaring (raw)
echo "$input" | jq '.'

# === TEST META ===
# @test:stdout: Ali
# @test:exit: 0
