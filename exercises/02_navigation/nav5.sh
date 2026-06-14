#!/usr/bin/env bash
#
# MASHQ: Wildcards bilan filterlash
# DARAJA: ★★☆☆☆
# MAVZU: part1/02-navigation · ls + wildcards
#
# Katalogda 3 ta fayl bor: a.txt, b.txt, c.log
# Faqat .txt fayllarini ko'rsating.
#
# Maslahat: `*` istalgan belgi(lar)ni anglatadi. `*.txt` — barcha .txt fayllar.
#
# --- English ---
# TASK: Filter with wildcards
# LEVEL: ★★☆☆☆
# TOPIC: part1/02-navigation · ls + wildcards
#
# The directory has 3 files: a.txt, b.txt, c.log
# Show only the .txt files.
#
# Hint: `*` means any character(s). `*.txt` — all .txt files.

# I AM NOT DONE

work=/tmp/bashlings-nav5
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
touch a.txt b.txt c.log

# TODO: faqat .txt bilan tugaydigan fayllarni listing qiling

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' a.txt b.txt
# @test:exit: 0
