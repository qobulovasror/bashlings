#!/usr/bin/env bash
# SOLUTION: text1 — head -n
work=/tmp/bashlings-text1
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
seq 1 10 > log.txt

head -n 3 log.txt

# === TEST META ===
# @test:stdout-cmd: printf '1\n2\n3\n'
# @test:exit: 0
