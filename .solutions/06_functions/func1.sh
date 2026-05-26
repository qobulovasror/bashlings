#!/usr/bin/env bash
# SOLUTION: func1 — birinchi funksiya
greet() {
    echo "Salom, $1!"
}

greet "Ali"

# === TEST META ===
# @test:stdout: Salom, Ali!
# @test:exit: 0
