#!/usr/bin/env bash
# SOLUTION: func3 — default argument
greet() {
    local name="${1:-Anonim}"
    echo "Salom, $name!"
}

greet "Ali"
greet

# === TEST META ===
# @test:stdout-cmd: printf 'Salom, Ali!\nSalom, Anonim!\n'
# @test:exit: 0
