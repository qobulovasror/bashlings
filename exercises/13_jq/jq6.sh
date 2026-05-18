#!/usr/bin/env bash
#
# MASHQ: Yig'indi
# DARAJA: ★★★★☆
# MAVZU: part3/03-jq · add
#
# Mahsulotlar massividan ularning narxlari YIG'INDISINI chiqaring.
#
# Input:    [{"price":10},{"price":20},{"price":30},{"price":15}]
# Kutilgan: 75   (10+20+30+15)
#
# Maslahat:
#   - `[.[] | .price]` — narxlar massivi
#   - `add` — array elementlari yig'indisi
#   - Yoki: `map(.price) | add`

# I AM NOT DONE

input='[{"price":10},{"price":20},{"price":30},{"price":15}]'

# TODO: narxlar yig'indisini chiqaring
echo "$input" | jq '.'

# === TEST META ===
# @test:stdout: 75
# @test:exit: 0
