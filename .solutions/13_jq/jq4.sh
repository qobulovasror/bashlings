#!/usr/bin/env bash
# SOLUTION: jq4 — object construction
input='{"firstName":"Ali","lastName":"Karim"}'
echo "$input" | jq -c '{fullName: (.firstName + " " + .lastName)}'

# === TEST META ===
# @test:stdout: {"fullName":"Ali Karim"}
# @test:exit: 0
