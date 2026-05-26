#!/usr/bin/env bash
# SOLUTION: sed1
work=/tmp/bashlings-sed1
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf 'old wine\nold tale\nold music\n' > input.txt

sed 's/old/new/' input.txt

# === TEST META ===
# @test:stdout-cmd: printf 'new wine\nnew tale\nnew music\n'
# @test:exit: 0
