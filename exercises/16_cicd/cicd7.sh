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
