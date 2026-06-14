#!/usr/bin/env bash
#
# MASHQ: Massivni e'lon qilish va chiqarish
# DARAJA: ★★☆☆☆
# MAVZU: part2/02-arrays · "${arr[@]}"
#
# `fruits` massivida 3 ta meva bor.
# Hamma elementlarni bitta echo orqali, probel bilan ajratib chiqaring:
#     olma anor uzum
#
# Maslahat:
#   - `"${arr[@]}"` — hamma elementlar (alohida argumentlar)
#   - `$fruits` — XATO: faqat birinchi elementni qaytaradi
#   - Har doim qo'shtirnoq ichida `"${arr[@]}"` yozing
#
# --- English ---
# TASK: Declare an array and print it
# LEVEL: ★★☆☆☆
# TOPIC: part2/02-arrays · "${arr[@]}"
#
# The `fruits` array has 3 fruits.
# Print all elements with a single echo, separated by spaces:
#     olma anor uzum
#
# Hint:
#   - `"${arr[@]}"` — all elements (as separate arguments)
#   - `$fruits` — WRONG: returns only the first element
#   - Always write `"${arr[@]}"` inside double quotes

# I AM NOT DONE

fruits=("olma" "anor" "uzum")

# TODO: hamma elementlarni chiqaring
echo "$fruits"   # bug: faqat olma

# === TEST META ===
# @test:stdout: olma anor uzum
# @test:exit: 0
