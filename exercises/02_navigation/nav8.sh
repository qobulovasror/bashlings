#!/usr/bin/env bash
#
# MASHQ: Faylni o'chirish
# DARAJA: ★★★☆☆
# MAVZU: part1/02-navigation · rm
#
# Katalogda 3 ta fayl bor: tmp1.txt, tmp2.txt, tmp3.txt.
# tmp2.txt ni o'chiring. Qolgan ikkita faylni `ls` bilan ko'rsating.
#
# ⚠ Eslatma: `rm` o'chirilgan faylni qaytarib bo'lmaydi! Diqqat bilan ishlating.
#
# --- English ---
# TASK: Delete a file
# LEVEL: ★★★☆☆
# TOPIC: part1/02-navigation · rm
#
# The directory has 3 files: tmp1.txt, tmp2.txt, tmp3.txt.
# Delete tmp2.txt. Show the remaining two files with `ls`.
#
# ⚠ Note: a file deleted by `rm` cannot be recovered! Use it carefully.

# I AM NOT DONE

work=/tmp/bashlings-nav8
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
touch tmp{1,2,3}.txt

# TODO: tmp2.txt ni o'chiring

# TODO: ls bilan qolganlarini ko'rsating

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' tmp1.txt tmp3.txt
# @test:exit: 0
