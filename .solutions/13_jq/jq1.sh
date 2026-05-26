#!/usr/bin/env bash
# SOLUTION: jq1 — field access
input='{"name":"Ali","age":25,"city":"Toshkent"}'
echo "$input" | jq -r '.name'

# === TEST META ===
# @test:stdout: Ali
# @test:exit: 0
