#!/usr/bin/env bash
#
# MASHQ: Workflow log'idan FAIL bo'lgan step'larni topish
# DARAJA: ★★★★☆
# MAVZU: part3/06-cicd · log parsing
#
# Quyida CI run'ning mock log'i. Faqat MUVAFFAQIYATSIZ ("FAIL") bo'lgan
# step'larning nomlarini chiqaring (alifbo tartibida):
#
#     Build
#     Type check
#
# Maslahat:
#   - Format: `<emoji> <STATUS> <stepname>`
#   - `grep '^✗'` yoki `grep FAIL` — fail satrlarini
#   - `awk -F'FAIL  ' '{print $2}'` — "FAIL<2 bo'sh joy>" dan keyingisi
#   - Yoki: `sed -E 's/^✗ FAIL  //'`
#   - Oxirida: `sort`
#
# --- English ---
# TASK: Finding FAILed steps from a workflow log
# LEVEL: ★★★★☆
# TOPIC: part3/06-cicd · log parsing
#
# Below is a mock log of a CI run. Print only the names of the steps that
# FAILED ("FAIL"), in alphabetical order:
#
#     Build
#     Type check
#
# Hint:
#   - Format: `<emoji> <STATUS> <stepname>`
#   - `grep '^✗'` or `grep FAIL` — the fail lines
#   - `awk -F'FAIL  ' '{print $2}'` — what comes after "FAIL<2 spaces>"
#   - Or: `sed -E 's/^✗ FAIL  //'`
#   - At the end: `sort`

# I AM NOT DONE

log='✓ PASS  Checkout code
✓ PASS  Setup Node.js
✗ FAIL  Type check
✓ PASS  Lint
✗ FAIL  Build
✓ PASS  Test'

# TODO: faqat FAIL step nomlarini alifbo tartibida chiqaring
echo "$log"

# === TEST META ===
# @test:stdout-cmd: printf 'Build\nType check\n'
# @test:exit: 0
