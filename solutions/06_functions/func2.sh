#!/usr/bin/env bash
# SOLUTION: func2 — echo return
square() {
    echo $(( $1 * $1 ))
}

result=$(square 5)
echo "5 ning kvadrati: $result"

# === TEST META ===
# @test:stdout: 5 ning kvadrati: 25
# @test:exit: 0
