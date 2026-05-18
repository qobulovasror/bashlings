#!/usr/bin/env bash
#
# MASHQ: awk bilan ustun yig'indisi
# DARAJA: ★★★★☆
# MAVZU: part2/03-sed-awk · sum + END block
#
# scores.txt:
#     ali 80
#     vali 75
#     gulnora 90
#
# Ikkinchi ustun (ballar) YIG'INDISINI chiqaring: 245
#
# Maslahat:
#   - `awk '{sum += $2} END {print sum}'`
#   - END bloki barcha qatorlar o'qib bo'lingach ishlaydi

# I AM NOT DONE

work=/tmp/bashlings-awk2
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf 'ali 80\nvali 75\ngulnora 90\n' > scores.txt

# TODO: 2-ustun yig'indisini awk bilan

# === TEST META ===
# @test:stdout: 245
# @test:exit: 0
