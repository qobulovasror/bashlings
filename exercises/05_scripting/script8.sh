#!/usr/bin/env bash
#
# MASHQ: Standart kirishdan o'qish
# DARAJA: ★★★☆☆
# MAVZU: part1/05-basic-scripting · read
#
# Skriptning stdin'iga uchta ism beriladi (har biri alohida qatorda):
#
#     Ali
#     Vali
#     Guli
#
# Birinchi qatorni `read` bilan o'qing va salomlashing,
# qolganlarini esa sanang:
#
#     Salom, Ali!
#     Qolganlari: 2
#
# Maslahat:
#   - `read -r name` — stdin'dan bitta qator o'qiydi
#   - `-r` teskari chiziqni maxsus belgi deb hisoblamaydi (deyarli har doim kerak)
#   - `read` faqat bitta qator oladi — qolgani stdin'da qoladi
#   - qolganini `while read -r _; do ...; done` bilan sanash mumkin
#
# --- English ---
# TASK: Read from standard input
# LEVEL: ★★★☆☆
# TOPIC: part1/05-basic-scripting · read
#
# Three names are fed to the script's stdin (one per line):
#
#     Ali
#     Vali
#     Guli
#
# Read the first line with `read` and greet it, then count the rest:
#
#     Salom, Ali!
#     Qolganlari: 2
#
# Hint:
#   - `read -r name` — reads one line from stdin
#   - `-r` stops backslash from being special (you almost always want it)
#   - `read` consumes only one line — the rest stays on stdin
#   - count the rest with `while read -r _; do ...; done`

# I AM NOT DONE

# TODO: birinchi qatorni o'qing va "Salom, <ism>!" deb chiqaring

# TODO: qolgan qatorlarni sanang va "Qolganlari: N" deb chiqaring

# === SETUP (qo'l urmang) ===
# @setup:stdin:
# |Ali
# |Vali
# |Guli

# === TEST META ===
# @test:stdout-cmd: printf 'Salom, Ali!\nQolganlari: 2\n'
# @test:exit: 0
