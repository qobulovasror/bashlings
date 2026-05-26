#!/usr/bin/env bash
# SOLUTION: script4 — for loop
for i in {1..5}; do
    echo "$i"
done

# === TEST META ===
# @test:stdout-cmd: seq 1 5
# @test:exit: 0
