#!/usr/bin/env bash
# SOLUTION: awk1
work=/tmp/bashlings-awk1
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf 'ali 25 toshkent\nvali 30 samarqand\n' > data.txt

awk '{print $3}' data.txt

# === TEST META ===
# @test:stdout-cmd: printf 'toshkent\nsamarqand\n'
# @test:exit: 0
