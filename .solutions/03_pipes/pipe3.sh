#!/usr/bin/env bash
# SOLUTION: pipe3 — | + tr
echo "salom dunyo bash" | tr ' ' '\n'

# === TEST META ===
# @test:stdout-cmd: printf 'salom\ndunyo\nbash\n'
# @test:exit: 0
