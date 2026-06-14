#!/usr/bin/env bash
#
# MASHQ: Object yaratish
# DARAJA: ★★★★☆
# MAVZU: part3/03-jq · {a: .b} + -c (compact)
#
# firstName va lastName dan yangi object yarating — fullName field bilan:
#
# Input:    {"firstName":"Ali","lastName":"Karim"}
# Kutilgan: {"fullName":"Ali Karim"}
#
# DIQQAT: natija COMPACT (bir qator) ko'rinishda. `-c` flag kerak.
#
# Maslahat:
#   - `{fullName: (.firstName + " " + .lastName)}` — yangi object
#   - String concatenation: `.a + " " + .b`
#   - `-c` — pretty-print emas, bir qator
#
# --- English ---
# TASK: Create an object
# LEVEL: ★★★★☆
# TOPIC: part3/03-jq · {a: .b} + -c (compact)
#
# From firstName and lastName, create a new object — with a fullName field:
#
# Input:    {"firstName":"Ali","lastName":"Karim"}
# Expected: {"fullName":"Ali Karim"}
#
# NOTE: the result must be COMPACT (one line). The `-c` flag is needed.
#
# Hint:
#   - `{fullName: (.firstName + " " + .lastName)}` — a new object
#   - String concatenation: `.a + " " + .b`
#   - `-c` — not pretty-print, one line

# I AM NOT DONE

input='{"firstName":"Ali","lastName":"Karim"}'

# TODO: jq -c bilan {"fullName":"Ali Karim"} yarating
echo "$input" | jq '.'

# === TEST META ===
# @test:stdout: {"fullName":"Ali Karim"}
# @test:exit: 0
