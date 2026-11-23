#!/usr/bin/env bash
# SOLUTION: pipe6 — sort | uniq
printf 'olma\nuzum\nolma\nbanan\nuzum\n' > fruits.txt

sort fruits.txt | uniq

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' banan olma uzum
# @test:exit: 0
