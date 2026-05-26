#!/usr/bin/env bash
# SOLUTION: arr4
nums=(10 20 30)
for i in "${!nums[@]}"; do
    echo "Element $((i+1)): ${nums[$i]}"
done

# === TEST META ===
# @test:stdout-cmd: printf 'Element 1: 10\nElement 2: 20\nElement 3: 30\n'
# @test:exit: 0
