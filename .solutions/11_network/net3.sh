#!/usr/bin/env bash
# SOLUTION: net3 — retry + timeout bilan curl
URL="https://api.example.com/data"
echo "curl --retry 3 -m 10 -fsSL $URL"

# === TEST META ===
# @test:stdout: curl --retry 3 -m 10 -fsSL https://api.example.com/data
# @test:exit: 0
