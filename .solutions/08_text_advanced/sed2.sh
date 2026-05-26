#!/usr/bin/env bash
# SOLUTION: sed2
work=/tmp/bashlings-sed2
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf 'qator-1\nqator-2\nqator-3\nqator-4\nqator-5\n' > log.txt

sed '2d;4d' log.txt

# === TEST META ===
# @test:stdout-cmd: printf 'qator-1\nqator-3\nqator-5\n'
# @test:exit: 0
