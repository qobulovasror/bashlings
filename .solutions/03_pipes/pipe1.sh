#!/usr/bin/env bash
# SOLUTION: pipe1 — > redirect

echo "salom" > hello.txt
cat hello.txt

# === TEST META ===
# @test:stdout: salom
# @test:exit: 0
