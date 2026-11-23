#!/usr/bin/env bash
# SOLUTION: text5 — head | tail (o'rta qatorlar)
seq 1 100 > big.txt

head -n 7 big.txt | tail -n 3

# === TEST META ===
# @test:stdout-cmd: printf '5\n6\n7\n'
# @test:exit: 0
