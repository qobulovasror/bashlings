#!/usr/bin/env bash
# SOLUTION: sed3
work=/tmp/bashlings-sed3
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf '2026-05-17\n2024-01-01\n' > date.txt

sed -E 's|([0-9]+)-([0-9]+)-([0-9]+)|\3/\2/\1|' date.txt

# === TEST META ===
# @test:stdout-cmd: printf '17/05/2026\n01/01/2024\n'
# @test:exit: 0
