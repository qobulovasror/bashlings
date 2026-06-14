#!/usr/bin/env bash
#
# MASHQ: if/else — yoshni tasniflash
# DARAJA: ★★★☆☆
# MAVZU: part1/05-basic-scripting · if/else
#
# `yosh` o'zgaruvchisi 20 ga teng.
# Agar `yosh > 18` bo'lsa: "kattalar" deb chiqaring.
# Aks holda: "yoshlar" deb chiqaring.
#
# Maslahat:
#   - `[[ $yosh -gt 18 ]]` — bash sonli taqqoslash
#   - Operatorlar: -gt (>), -lt (<), -ge (>=), -le (<=), -eq (==), -ne (!=)
#   - Sintaksis:
#         if [[ shart ]]; then
#             ...
#         else
#             ...
#         fi
#
# --- English ---
# TASK: if/else — classify age
# LEVEL: ★★★☆☆
# TOPIC: part1/05-basic-scripting · if/else
#
# The `yosh` variable equals 20.
# If `yosh > 18`: print "kattalar".
# Otherwise: print "yoshlar".
#
# Hint:
#   - `[[ $yosh -gt 18 ]]` — bash numeric comparison
#   - Operators: -gt (>), -lt (<), -ge (>=), -le (<=), -eq (==), -ne (!=)
#   - Syntax:
#         if [[ condition ]]; then
#             ...
#         else
#             ...
#         fi

# I AM NOT DONE

yosh=20

# TODO: if/else bilan tasniflash

# === TEST META ===
# @test:stdout: kattalar
# @test:exit: 0
