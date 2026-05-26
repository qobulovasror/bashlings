#!/usr/bin/env bash
# SOLUTION: rob1
set -o pipefail

echo "ish"
false | echo "yashirildi"

# === TEST META ===
# @test:stdout-cmd: printf 'ish\nyashirildi\n'
# @test:exit: 1
