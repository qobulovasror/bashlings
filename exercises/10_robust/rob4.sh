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
