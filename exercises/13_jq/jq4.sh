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

# I AM NOT DONE

input='{"firstName":"Ali","lastName":"Karim"}'

# TODO: jq -c bilan {"fullName":"Ali Karim"} yarating
echo "$input" | jq '.'

# === TEST META ===
# @test:stdout: {"fullName":"Ali Karim"}
# @test:exit: 0
