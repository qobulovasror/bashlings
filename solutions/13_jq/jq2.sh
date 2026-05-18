#!/usr/bin/env bash
# SOLUTION: jq2 — array iteration
input='[{"name":"Ali"},{"name":"Vali"},{"name":"Gulnora"}]'
echo "$input" | jq -r '.[].name'

# === TEST META ===
# @test:stdout-cmd: printf 'Ali\nVali\nGulnora\n'
# @test:exit: 0
