#!/usr/bin/env bash
# SOLUTION: script3 — if/else
yosh=20

if [[ $yosh -gt 18 ]]; then
    echo "kattalar"
else
    echo "yoshlar"
fi

# === TEST META ===
# @test:stdout: kattalar
# @test:exit: 0
