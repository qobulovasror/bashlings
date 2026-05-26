#!/usr/bin/env bash
# SOLUTION: jq3 — filter + count
input='[{"age":15},{"age":20},{"age":17},{"age":30},{"age":22}]'
echo "$input" | jq '[.[] | select(.age >= 18)] | length'

# === TEST META ===
# @test:stdout: 3
# @test:exit: 0
