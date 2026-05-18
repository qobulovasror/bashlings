#!/usr/bin/env bash
# SOLUTION: trap2
cleanup() {
    echo "tozalandi"
}
trap cleanup EXIT

echo "asosiy ish"

# === TEST META ===
# @test:stdout-cmd: printf 'asosiy ish\ntozalandi\n'
# @test:exit: 0
