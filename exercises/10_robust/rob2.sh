#!/usr/bin/env bash
#
# MASHQ: O'zgaruvchini quote qilish
# DARAJA: ★★★☆☆
# MAVZU: part2/05-robust-scripting · "$var"
#
# Fayl nomida PROBEL bor. Quote'siz `ls $name` ikki argument deb tushuniladi
# va xato beradi.
#
# `$name` ni qo'shtirnoq ichiga oling: `"$name"` — bash bir argument deb biladi.
#
# Kutilgan: "Ali Vali.txt"
#
# --- English ---
# TASK: Quoting a variable
# LEVEL: ★★★☆☆
# TOPIC: part2/05-robust-scripting · "$var"
#
# The file name contains a SPACE. Without quotes, `ls $name` is read as two
# arguments and fails.
#
# Wrap `$name` in double quotes: `"$name"` — bash treats it as one argument.
#
# Expected: "Ali Vali.txt"

# I AM NOT DONE

work=/tmp/bashlings-rob2
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
touch "Ali Vali.txt"

name="Ali Vali.txt"

# TODO: $name ni quote ichiga oling
ls $name

# === TEST META ===
# @test:stdout: Ali Vali.txt
# @test:exit: 0
