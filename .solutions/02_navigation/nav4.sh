#!/usr/bin/env bash
# SOLUTION: nav4 — touch + brace expansion

touch file{1,2,3}.txt
ls

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' file1.txt file2.txt file3.txt
# @test:exit: 0
