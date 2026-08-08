#!/usr/bin/env bash
#
# MASHQ: Skript argumentlari
# DARAJA: ★★★☆☆
# MAVZU: part1/05-basic-scripting · $1, $#, $@
#
# Skript ikkita argument bilan chaqiriladi: "Ali" va "Vali Aliyev".
# Quyidagini aynan shu tartibda chiqaring:
#
#     Jami: 2
#     Birinchi: Ali
#     Barchasi: Ali | Vali Aliyev
#
# ⚠ Ikkinchi argumentda bo'shliq bor — tirnoqsiz "$@" ikkiga bo'linib ketadi.
#
# Maslahat:
#   - `$#` — argumentlar soni
#   - `$1` — birinchi argument
#   - `"$@"` — har bir argument alohida so'z bo'lib qoladi (tirnoq shart!)
#   - `printf '%s | %s\n' "$@"` — ikkita argumentni ajratgich bilan chiqaradi
#
# --- English ---
# TASK: Script arguments
# LEVEL: ★★★☆☆
# TOPIC: part1/05-basic-scripting · $1, $#, $@
#
# The script is called with two arguments: "Ali" and "Vali Aliyev".
# Print exactly this, in this order:
#
#     Jami: 2
#     Birinchi: Ali
#     Barchasi: Ali | Vali Aliyev
#
# ⚠ The second argument contains a space — unquoted, "$@" would split in two.
#
# Hint:
#   - `$#` — the number of arguments
#   - `$1` — the first argument
#   - `"$@"` — keeps every argument a single word (the quotes matter!)
#   - `printf '%s | %s\n' "$@"` — prints two arguments with a separator

# I AM NOT DONE

# TODO: argumentlar sonini chiqaring — "Jami: N"

# TODO: birinchi argumentni chiqaring — "Birinchi: ..."

# TODO: barchasini " | " ajratgichi bilan chiqaring — "Barchasi: ..."

# === SETUP (qo'l urmang) ===
# @setup:args: Ali "Vali Aliyev"

# === TEST META ===
# @test:stdout-cmd: printf 'Jami: 2\nBirinchi: Ali\nBarchasi: Ali | Vali Aliyev\n'
# @test:exit: 0
