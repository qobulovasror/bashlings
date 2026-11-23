#!/usr/bin/env bash
# SOLUTION: pipe7 — tee

echo "salom" | tee out.txt

# === TEST META ===
# @test:stdout: salom
# @test:exit: 0
