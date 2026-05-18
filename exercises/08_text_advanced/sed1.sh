#!/usr/bin/env bash
#
# MASHQ: sed substitution
# DARAJA: ★★★☆☆
# MAVZU: part2/03-sed-awk · s/old/new/
#
# input.txt har qatorida "old" so'zi bor.
# `sed` ishlatib hammasini "new" ga almashtiring.
#
# Kutilgan:
#     new wine
#     new tale
#     new music

# I AM NOT DONE

work=/tmp/bashlings-sed1
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf 'old wine\nold tale\nold music\n' > input.txt

# TODO: sed bilan "old" -> "new" almashtiring

# === TEST META ===
# @test:stdout-cmd: printf 'new wine\nnew tale\nnew music\n'
# @test:exit: 0
