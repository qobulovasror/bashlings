#!/usr/bin/env bash
#
# MASHQ: Bir nechta katalog yaratish
# DARAJA: ★★☆☆☆
# MAVZU: part1/02-navigation · mkdir -p
#
# Quyidagi 3 ta katalog yarating: docs, src, tests
# Keyin ularni `ls` orqali ko'rsating.
#
# Maslahat: `mkdir` bir vaqtda bir nechta argumentni qabul qiladi.
#           `-p` flagi yo'l elementlarini ham yaratadi.
#
# --- English ---
# TASK: Create several directories
# LEVEL: ★★☆☆☆
# TOPIC: part1/02-navigation · mkdir -p
#
# Create the following 3 directories: docs, src, tests
# Then list them with `ls`.
#
# Hint: `mkdir` accepts several arguments at once.
#           the `-p` flag also creates the path components.

# I AM NOT DONE

work=/tmp/bashlings-nav3
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1

# TODO: bu yerda 3 ta katalogni yarating

ls

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' docs src tests
# @test:exit: 0
