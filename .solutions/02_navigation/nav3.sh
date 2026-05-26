#!/usr/bin/env bash
# SOLUTION: nav3 — bir nechta katalog
work=/tmp/bashlings-nav3
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1

mkdir -p docs src tests
ls

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' docs src tests
# @test:exit: 0
