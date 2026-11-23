#!/usr/bin/env bash
# SOLUTION: awk1
printf 'ali 25 toshkent\nvali 30 samarqand\n' > data.txt

awk '{print $3}' data.txt

# === TEST META ===
# @test:stdout-cmd: printf 'toshkent\nsamarqand\n'
# @test:exit: 0
