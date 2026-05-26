#!/usr/bin/env bash
# SOLUTION: pipe4 — seq | tail
seq 1 10 | tail -n 3

# === TEST META ===
# @test:stdout-cmd: printf '8\n9\n10\n'
# @test:exit: 0
