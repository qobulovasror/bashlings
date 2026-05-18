#!/usr/bin/env bash
# SOLUTION: arr6
csv="Ali,Vali,Gulnora"
IFS=',' read -ra names <<< "$csv"
echo "${names[1]}"

# === TEST META ===
# @test:stdout: Vali
# @test:exit: 0
