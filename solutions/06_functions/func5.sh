#!/usr/bin/env bash
# SOLUTION: func5 — funksiyalar kompozitsiyasi
add() {
    echo $(( $1 + $2 ))
}

multiply() {
    echo $(( $1 * $2 ))
}

sum=$(add 3 4)
result=$(multiply "$sum" 2)
echo "$result"

# === TEST META ===
# @test:stdout: 14
# @test:exit: 0
