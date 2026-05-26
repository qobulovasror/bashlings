#!/usr/bin/env bash
# SOLUTION: text5 — head | tail (o'rta qatorlar)
work=/tmp/bashlings-text5
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
seq 1 100 > big.txt

head -n 7 big.txt | tail -n 3

# === TEST META ===
# @test:stdout-cmd: printf '5\n6\n7\n'
# @test:exit: 0
