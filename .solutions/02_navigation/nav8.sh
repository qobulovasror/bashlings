#!/usr/bin/env bash
# SOLUTION: nav8 — rm + qolganlar
rm tmp2.txt
ls

# === SETUP (qo'l urmang) ===
# @setup:file: tmp1.txt
# @setup:file: tmp2.txt
# @setup:file: tmp3.txt

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' tmp1.txt tmp3.txt
# @test:exit: 0
