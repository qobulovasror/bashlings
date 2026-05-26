#!/usr/bin/env bash
# SOLUTION: trap4
tmpfile=$(mktemp)
echo "data" > "$tmpfile"

cleanup() {
    rm -f "$tmpfile"
    echo "tmp o'chirildi"
}
trap cleanup EXIT

echo "asosiy ish"

# === TEST META ===
# @test:stdout-cmd: printf "asosiy ish\ntmp o'chirildi\n"
# @test:exit: 0
