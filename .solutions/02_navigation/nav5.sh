#!/usr/bin/env bash
# SOLUTION: nav5 — wildcards
ls *.txt

# === SETUP (qo'l urmang) ===
# @setup:file: a.txt
# @setup:file: b.txt
# @setup:file: c.log

# === TEST META ===
# @test:stdout-cmd: printf '%s\n' a.txt b.txt
# @test:exit: 0
