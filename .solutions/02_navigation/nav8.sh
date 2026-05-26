#!/usr/bin/env bash
# SOLUTION: nav8 — rm + qolganlar
work=/tmp/bashlings-nav8
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
touch tmp{1,2,3}.txt

rm tmp2.txt
ls

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' tmp1.txt tmp3.txt
# @test:exit: 0
