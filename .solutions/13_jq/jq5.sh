#!/usr/bin/env bash
# SOLUTION: jq5 — max_by
input='[{"name":"A","score":80},{"name":"B","score":95},{"name":"C","score":70}]'
echo "$input" | jq -r 'max_by(.score) | .name'

# === TEST META ===
# @test:stdout: B
# @test:exit: 0
