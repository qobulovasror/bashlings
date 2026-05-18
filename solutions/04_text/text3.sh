#!/usr/bin/env bash
# SOLUTION: text3 — sort -n
work=/tmp/bashlings-text3
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf '42\n5\n100\n7\n23\n' > scores.txt

sort -n scores.txt

# === TEST META ===
# @test:stdout-cmd: printf '5\n7\n23\n42\n100\n'
# @test:exit: 0
