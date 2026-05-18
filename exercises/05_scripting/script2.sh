#!/usr/bin/env bash
#
# MASHQ: Command substitution
# DARAJA: ★★★☆☆
# MAVZU: part1/05-basic-scripting · $(...)
#
# Joriy katalogning to'liq yo'li va uning oxirgi qismini bitta qatorda chiqaring:
#     /Users/mac (mac)
#     /home/ali (ali)
#
# Ya'ni format: "<to'liq_yo'l> (<oxirgi_qism>)"
#
# Maslahat:
#   - $PWD — joriy katalog yo'li (environment variable)
#   - `basename "/path/to/x"` → "x" (oxirgi qismni qaytaradi)
#   - $(...) — buyruq natijasini matnga aylantirib qo'shadi
#
# Eslatma: bu mashq juda kuchli pattern — environment variable + command substitution.

# I AM NOT DONE

# TODO: $PWD va $(basename ...) ni birlashtirib chiqaring

# === TEST META ===
# @test:stdout-cmd: echo "$PWD ($(basename "$PWD"))"
# @test:exit: 0
