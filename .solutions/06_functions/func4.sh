#!/usr/bin/env bash
# SOLUTION: func4 — local scope
inner() {
    local name="ichki"
    echo "Funksiya ichida: $name"
}

name="tashqi"
inner
echo "Funksiya tashqarisida: $name"

# === TEST META ===
# @test:stdout-cmd: printf 'Funksiya ichida: ichki\nFunksiya tashqarisida: tashqi\n'
# @test:exit: 0
