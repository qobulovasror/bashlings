#!/usr/bin/env bash
#
# MASHQ: Brace expansion bilan fayllar yaratish
# DARAJA: ★★☆☆☆
# MAVZU: part1/02-navigation · touch + brace expansion
#
# 3 ta fayl yarating — file1.txt, file2.txt, file3.txt — bitta buyruqda.
# Keyin ularni listing qiling.
#
# Maslahat: brace expansion → `{1,2,3}` shell uchun "1 2 3" ga ochiladi.
# Yoki diapazon shaklida: `{1..3}`.
#
# --- English ---
# TASK: Create files with brace expansion
# LEVEL: ★★☆☆☆
# TOPIC: part1/02-navigation · touch + brace expansion
#
# Create 3 files — file1.txt, file2.txt, file3.txt — in a single command.
# Then list them.
#
# Hint: brace expansion → `{1,2,3}` expands to "1 2 3" for the shell.
# Or as a range: `{1..3}`.

# I AM NOT DONE

work=/tmp/bashlings-nav4
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1

# TODO: bitta touch chaqiruvi bilan 3 ta faylni yarating

ls

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' file1.txt file2.txt file3.txt
# @test:exit: 0
