#!/usr/bin/env bash
# SOLUTION: nav6 — cp + cat
work=/tmp/bashlings-nav6
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
echo "salom dunyo" > manba.txt

cp manba.txt nusxa.txt
cat nusxa.txt

# === TEST META ===
# @test:stdout: salom dunyo
# @test:exit: 0
