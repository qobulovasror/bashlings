#!/usr/bin/env bash
# SOLUTION: pipe2 — >> append
work=/tmp/bashlings-pipe2
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
echo "birinchi" > log.txt

echo "ikkinchi" >> log.txt
cat log.txt

# === TEST META ===
# @test:stdout-cmd: printf 'birinchi\nikkinchi\n'
# @test:exit: 0
