#!/usr/bin/env bash
# SOLUTION: script5 — case
tugma="b"

case "$tugma" in
    a) echo "Birinchi" ;;
    b) echo "Ikkinchi" ;;
    c) echo "Uchinchi" ;;
    *) echo "Noma'lum" ;;
esac

# === TEST META ===
# @test:stdout: Ikkinchi
# @test:exit: 0
