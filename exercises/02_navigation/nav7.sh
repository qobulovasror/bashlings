#!/usr/bin/env bash
#
# MASHQ: Faylni qayta nomlash
# DARAJA: ★★☆☆☆
# MAVZU: part1/02-navigation · mv
#
# Katalogda eski.txt fayli bor. Uning nomini yangi.txt ga o'zgartiring.
# Keyin `ls` orqali katalog tarkibini ko'rsating.
#
# Maslahat: `mv <eski-nom> <yangi-nom>` — bir buyruqda ham ko'chiradi, ham rename qiladi.

# I AM NOT DONE

work=/tmp/bashlings-nav7
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
touch eski.txt

# TODO: eski.txt ni yangi.txt ga rename qiling

# TODO: ls

# === TEST META ===
# @test:stdout: yangi.txt
# @test:exit: 0
