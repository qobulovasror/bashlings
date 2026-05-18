#!/usr/bin/env bash
#
# MASHQ: Nusxa olish
# DARAJA: ★★☆☆☆
# MAVZU: part1/02-navigation · cp
#
# manba.txt ichida "salom dunyo" matni bor.
# Faylni nusxa.txt nomi bilan nusxa oling.
# Keyin nusxa.txt ichidagi matnni `cat` orqali chiqaring.
#
# Maslahat: `cp <manba> <maqsad>`

# I AM NOT DONE

work=/tmp/bashlings-nav6
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
echo "salom dunyo" > manba.txt

# TODO: cp orqali nusxa.txt yarating

# TODO: nusxa.txt ni cat bilan chiqaring

# === TEST META ===
# @test:stdout: salom dunyo
# @test:exit: 0
