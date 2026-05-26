#!/usr/bin/env bash
# SOLUTION: script6 — while counter
i=1
while [[ $i -le 3 ]]; do
    echo "Qadam $i"
    ((i++))
done

# === TEST META ===
# @test:stdout-cmd: printf 'Qadam 1\nQadam 2\nQadam 3\n'
# @test:exit: 0
