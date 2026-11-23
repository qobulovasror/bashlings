#!/usr/bin/env bash
# SOLUTION: pipe2 — >> append
echo "birinchi" > log.txt

echo "ikkinchi" >> log.txt
cat log.txt

# === TEST META ===
# @test:stdout-cmd: printf 'birinchi\nikkinchi\n'
# @test:exit: 0
