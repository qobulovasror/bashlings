#!/usr/bin/env bash
# SOLUTION: cicd7 — fail step nomlarini ajratish
log='✓ PASS  Checkout code
✓ PASS  Setup Node.js
✗ FAIL  Type check
✓ PASS  Lint
✗ FAIL  Build
✓ PASS  Test'

echo "$log" | grep '^✗' | sed -E 's/^✗ FAIL  //' | sort

# === TEST META ===
# @test:stdout-cmd: printf 'Build\nType check\n'
# @test:exit: 0
