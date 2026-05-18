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

# I AM NOT DONE

fruits=("olma" "anor" "uzum")

# TODO: hamma elementlarni chiqaring
echo "$fruits"   # bug: faqat olma

# === TEST META ===
# @test:stdout: olma anor uzum
# @test:exit: 0
