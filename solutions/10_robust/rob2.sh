#!/usr/bin/env bash
# SOLUTION: rob2
work=/tmp/bashlings-rob2
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
touch "Ali Vali.txt"

name="Ali Vali.txt"
ls "$name"

# === TEST META ===
# @test:stdout: Ali Vali.txt
# @test:exit: 0
