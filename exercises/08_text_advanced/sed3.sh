#!/usr/bin/env bash
#
# MASHQ: Sana formatini o'zgartirish (backreference)
# DARAJA: ★★★★☆
# MAVZU: part2/03-sed-awk · sed -E + backreferences
#
# date.txt ichida ISO sanalar (YYYY-MM-DD):
#     2026-05-17
#     2024-01-01
#
# Formatni DD/MM/YYYY ga o'zgartiring:
#     17/05/2026
#     01/01/2024
#
# Maslahat:
#   - `sed -E 's/(YEAR)-(MONTH)-(DAY)/\3\/\2\/\1/'`
#   - `-E` — extended regex (`(...)` guruh, `+`, `?`)
#   - `\3`, `\2`, `\1` — orqaga reference
#   - `/` ni `\` bilan escape qilish kerak (yoki ajratuvchini almashtirib `s|...|...|` ishlatish)
#
# --- English ---
# TASK: Reformatting a date (backreference)
# LEVEL: ★★★★☆
# TOPIC: part2/03-sed-awk · sed -E + backreferences
#
# date.txt contains ISO dates (YYYY-MM-DD):
#     2026-05-17
#     2024-01-01
#
# Change the format to DD/MM/YYYY:
#     17/05/2026
#     01/01/2024
#
# Hint:
#   - `sed -E 's/(YEAR)-(MONTH)-(DAY)/\3\/\2\/\1/'`
#   - `-E` — extended regex (`(...)` group, `+`, `?`)
#   - `\3`, `\2`, `\1` — backreferences
#   - `/` must be escaped with `\` (or change the delimiter and use `s|...|...|`)

# I AM NOT DONE

work=/tmp/bashlings-sed3
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf '2026-05-17\n2024-01-01\n' > date.txt

# TODO: sed -E bilan format o'zgartirish

# === TEST META ===
# @test:stdout-cmd: printf '17/05/2026\n01/01/2024\n'
# @test:exit: 0
