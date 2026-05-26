#!/usr/bin/env bash
# SOLUTION: nav7 — mv rename
work=/tmp/bashlings-nav7
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
touch eski.txt

mv eski.txt yangi.txt
ls

# === TEST META ===
# @test:stdout: yangi.txt
# @test:exit: 0
