#!/usr/bin/env bash
# SOLUTION: trap1
trap 'echo "tozalandi"' EXIT
echo "ishlayapti"

# === TEST META ===
# @test:stdout-cmd: printf 'ishlayapti\ntozalandi\n'
# @test:exit: 0
