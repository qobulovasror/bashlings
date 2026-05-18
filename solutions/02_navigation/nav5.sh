#!/usr/bin/env bash
# SOLUTION: nav5 — wildcards
work=/tmp/bashlings-nav5
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
touch a.txt b.txt c.log

ls *.txt

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' a.txt b.txt
# @test:exit: 0
