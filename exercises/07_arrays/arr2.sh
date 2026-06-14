#!/usr/bin/env bash
#
# MASHQ: Element qo'shish va oxirgisini olish
# DARAJA: ★★★☆☆
# MAVZU: part2/02-arrays · arr+=(...) va ${arr[-1]}
#
# `colors` massivida 2 ta rang bor. Unga "yashil" rangini qo'shing.
# Keyin OXIRGI elementni chiqaring (kutilgan: yashil).
#
# Maslahat:
#   - Qo'shish: `arr+=("element")`
#   - Oxirgi element: `${arr[-1]}` (Bash 4.2+)
#   - Yoki: `${arr[${#arr[@]}-1]}` (eski bash)
#
# --- English ---
# TASK: Add an element and get the last one
# LEVEL: ★★★☆☆
# TOPIC: part2/02-arrays · arr+=(...) and ${arr[-1]}
#
# The `colors` array has 2 colors. Add the color "yashil" to it.
# Then print the LAST element (expected: yashil).
#
# Hint:
#   - Append: `arr+=("element")`
#   - Last element: `${arr[-1]}` (Bash 4.2+)
#   - Or: `${arr[${#arr[@]}-1]}` (older bash)

# I AM NOT DONE

colors=("qizil" "kok")

# TODO: yashil rangni qo'shing


# TODO: oxirgi elementni chiqaring


# === TEST META ===
# @test:stdout: yashil
# @test:exit: 0
