#!/usr/bin/env bash
# SOLUTION: nav4 — touch + brace expansion
work=/tmp/bashlings-nav4
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1

touch file{1,2,3}.txt
ls

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' file1.txt file2.txt file3.txt
# @test:exit: 0
