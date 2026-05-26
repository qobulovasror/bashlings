#!/usr/bin/env bash
# SOLUTION: arr5
declare -A user
user[name]="Ali"
user[city]="Toshkent"
echo "${user[name]} ${user[city]}"

# === TEST META ===
# @test:stdout: Ali Toshkent
# @test:exit: 0
