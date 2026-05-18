#!/usr/bin/env bash
# SOLUTION: pipe1 — > redirect
work=/tmp/bashlings-pipe1
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1

echo "salom" > hello.txt
cat hello.txt

# === TEST META ===
# @test:stdout: salom
# @test:exit: 0
