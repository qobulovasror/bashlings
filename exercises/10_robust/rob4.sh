#!/usr/bin/env bash
#
# MASHQ: Idempotent mkdir
# DARAJA: ★★★☆☆
# MAVZU: part2/05-robust-scripting · idempotent + mkdir -p
#
# Skript ikki marta `natija` katalogini yaratishga harakat qiladi.
# `set -e` ostida — ikkinchi `mkdir` skriptni to'xtatadi (xato).
#
# `-p` flagini qo'shing — qayta yaratish bezararsiz bo'lsin.
#
# Kutilgan: "tayyor"
#
# --- English ---
# TASK: Idempotent mkdir
# LEVEL: ★★★☆☆
# TOPIC: part2/05-robust-scripting · idempotent + mkdir -p
#
# The script tries to create the `natija` directory twice.
# Under `set -e` — the second `mkdir` stops the script (error).
#
# Add the `-p` flag — so recreating it is harmless.
#
# Expected: "tayyor"

set -e
work=natija
rm -rf "$work"

# TODO: mkdir -p ishlatib idempotent qiling
mkdir "$work"
mkdir "$work"

echo "tayyor"

# === TEST META ===
# @test:stdout: tayyor
# @test:exit: 0
