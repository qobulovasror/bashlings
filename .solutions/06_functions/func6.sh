#!/usr/bin/env bash
# SOLUTION: func6 — boolean funksiya
is_even() {
    return $(( $1 % 2 ))
}

for n in 4 7; do
    if is_even "$n"; then
        echo "$n juft"
    else
        echo "$n toq"
    fi
done

# === TEST META ===
# @test:stdout-cmd: printf '4 juft\n7 toq\n'
# @test:exit: 0
