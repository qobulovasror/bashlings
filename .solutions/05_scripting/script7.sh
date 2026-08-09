#!/usr/bin/env bash
# SOLUTION: script7 — $#, $1, "$@"

echo "Jami: $#"
echo "Birinchi: $1"
printf 'Barchasi: %s | %s\n' "$@"

# === SETUP (qo'l urmang) ===
# @setup:args: Ali "Vali Aliyev"

# === TEST META ===
# @test:stdout-cmd: printf 'Jami: 2\nBirinchi: Ali\nBarchasi: Ali | Vali Aliyev\n'
# @test:exit: 0
