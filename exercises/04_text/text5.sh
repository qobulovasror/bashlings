#!/usr/bin/env bash
#
# MASHQ: O'rta qatorlarni olish
# DARAJA: ★★★★☆
# MAVZU: part1/04-text-processing · head | tail pipeline
#
# big.txt da 1 dan 100 gacha sonlar (har qatorda bittadan).
# Faqat 5, 6, 7 — qatorlarini chiqaring.
#
# Maslahat:
#   - `head -n 7` — birinchi 7 qator (1..7)
#   - `tail -n 3` — oxirgi 3 qator (5, 6, 7)
#   - Ikki buyruqni `|` bilan bog'lang
#
# Boshqacha aytganda: avval 1-7 oraliqni kesib, keyin oxirgi 3 tasini olamiz.

# I AM NOT DONE

work=/tmp/bashlings-text5
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
seq 1 100 > big.txt

# TODO: big.txt'dan 5, 6, 7 qatorlarini head + tail orqali oling

# === TEST META ===
# @test:stdout-cmd: printf '5\n6\n7\n'
# @test:exit: 0
