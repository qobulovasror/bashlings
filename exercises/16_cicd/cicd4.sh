#!/usr/bin/env bash
#
# MASHQ: Matrix — bir nechta OS'da test
# DARAJA: ★★★☆☆
# MAVZU: part3/06-cicd · strategy.matrix
#
# Matrix — bir job'ni har xil parametrlar bilan PARALLEL ishlatish.
# Quyidagi blokni chiqaring:
#
#     strategy:
#       matrix:
#         os: [ubuntu-latest, macos-latest]
#     runs-on: ${{ matrix.os }}
#
# Diqqat:
#   - YAML inline list: `[a, b]` (komma + bo'sh joy)
#   - `${{ ... }}` — GitHub Actions expression syntax
#   - `runs-on:` o'sha matrix.os qiymatini oladi
#
# Eslatma: `${{ ... }}` ichidagi `$` — bash bunday qilib expand qilmaydi
# (chunki `{{ }}` ichida), lekin quote qilish kerak. Quyidagilarga e'tibor:
#   - `'${{ matrix.os }}'`  — bir tirnoq (eng ishonchli)
#   - `"\${{ matrix.os }}"` — qo'shtirnoq + escape
#
# --- English ---
# TASK: Matrix — testing on multiple OSes
# LEVEL: ★★★☆☆
# TOPIC: part3/06-cicd · strategy.matrix
#
# A matrix — running one job in PARALLEL with different parameters.
# Print the following block:
#
#     strategy:
#       matrix:
#         os: [ubuntu-latest, macos-latest]
#     runs-on: ${{ matrix.os }}
#
# Attention:
#   - YAML inline list: `[a, b]` (comma + space)
#   - `${{ ... }}` — GitHub Actions expression syntax
#   - `runs-on:` takes that matrix.os value
#
# Note: the `$` inside `${{ ... }}` — bash does not expand it this way
# (because it is inside `{{ }}`), but you still need to quote it. Pay attention to:
#   - `'${{ matrix.os }}'`  — single quotes (most reliable)
#   - `"\${{ matrix.os }}"` — double quotes + escape

# I AM NOT DONE

# TODO: yuqoridagi YAML'ni chiqaring
echo "strategy:"

# === TEST META ===
# @test:stdout-cmd: printf 'strategy:\n  matrix:\n    os: [ubuntu-latest, macos-latest]\nruns-on: ${{ matrix.os }}\n'
# @test:exit: 0
