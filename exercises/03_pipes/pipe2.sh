#!/usr/bin/env bash
#
# MASHQ: Faylga qo'shib yozish
# DARAJA: ★★☆☆☆
# MAVZU: part1/03-pipes-redirection · >> append
#
# log.txt allaqachon "birinchi" satrini saqlaydi.
# Faylga "ikkinchi" satrini QO'SHING (eski mazmunni yo'qotmasdan).
# Keyin to'liq mazmunini chiqaring.
#
# Maslahat: > qayta yozadi, >> qo'shib boradi.
#
# --- English ---
# TASK: Append to a file
# LEVEL: ★★☆☆☆
# TOPIC: part1/03-pipes-redirection · >> append
#
# log.txt already stores the line "birinchi".
# APPEND the line "ikkinchi" to the file (without losing the old contents).
# Then print the full contents.
#
# Hint: > overwrites, >> appends.

# I AM NOT DONE

work=/tmp/bashlings-pipe2
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
echo "birinchi" > log.txt

# TODO: log.txt'ga "ikkinchi" qatorini qo'shing

# TODO: faylni to'liq chiqaring

# === TEST META ===
# @test:stdout-cmd: printf 'birinchi\nikkinchi\n'
# @test:exit: 0
