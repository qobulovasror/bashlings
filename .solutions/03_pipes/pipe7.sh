#!/usr/bin/env bash
# SOLUTION: pipe7 — tee
work=/tmp/bashlings-pipe7
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1

echo "salom" | tee out.txt

# === TEST META ===
# @test:stdout: salom
# @test:exit: 0
