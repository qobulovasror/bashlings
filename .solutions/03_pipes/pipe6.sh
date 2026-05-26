#!/usr/bin/env bash
# SOLUTION: pipe6 — sort | uniq
work=/tmp/bashlings-pipe6
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf 'olma\nuzum\nolma\nbanan\nuzum\n' > fruits.txt

sort fruits.txt | uniq

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' banan olma uzum
# @test:exit: 0
