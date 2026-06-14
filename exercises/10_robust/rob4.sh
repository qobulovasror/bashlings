#!/usr/bin/env bash
#
# MASHQ: Idempotent mkdir
# DARAJA: ★★★☆☆
# MAVZU: part2/05-robust-scripting · idempotent + mkdir -p
#
# Skript ikki marta /tmp/bashlings-rob4 katalogini yaratishga harakat qiladi.
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
# The script tries to create the /tmp/bashlings-rob4 directory twice.
# Under `set -e` — the second `mkdir` stops the script (error).
#
# Add the `-p` flag — so recreating it is harmless.
#
# Expected: "tayyor"

# I AM NOT DONE

set -e
work=/tmp/bashlings-rob4
rm -rf "$work"

# TODO: mkdir -p ishlatib idempotent qiling
mkdir "$work"
mkdir "$work"

echo "tayyor"

# === TEST META ===
# @test:stdout: tayyor
# @test:exit: 0
