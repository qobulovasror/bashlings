#!/usr/bin/env bash
# SOLUTION: script2 — command substitution
echo "$PWD ($(basename "$PWD"))"

# === TEST META ===
# @test:stdout-cmd: echo "$PWD ($(basename "$PWD"))"
# @test:exit: 0
