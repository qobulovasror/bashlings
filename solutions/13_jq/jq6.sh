#!/usr/bin/env bash
# SOLUTION: jq6 — sum
input='[{"price":10},{"price":20},{"price":30},{"price":15}]'
echo "$input" | jq '[.[].price] | add'

# === TEST META ===
# @test:stdout: 75
# @test:exit: 0
