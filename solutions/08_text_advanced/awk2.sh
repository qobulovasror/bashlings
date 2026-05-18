#!/usr/bin/env bash
# SOLUTION: awk2
work=/tmp/bashlings-awk2
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf 'ali 80\nvali 75\ngulnora 90\n' > scores.txt

awk '{sum += $2} END {print sum}' scores.txt

# === TEST META ===
# @test:stdout: 245
# @test:exit: 0
