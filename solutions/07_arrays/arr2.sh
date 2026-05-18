#!/usr/bin/env bash
# SOLUTION: arr2
colors=("qizil" "kok")
colors+=("yashil")
echo "${colors[-1]}"

# === TEST META ===
# @test:stdout: yashil
# @test:exit: 0
