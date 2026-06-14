#!/usr/bin/env bash
#
# MASHQ: Birinchi pipeline
# DARAJA: ★★☆☆☆
# MAVZU: part1/03-pipes-redirection · | va tr
#
# "salom dunyo bash" matnini har so'zni ALOHIDA QATORDA chiqaring:
#     salom
#     dunyo
#     bash
#
# Maslahat:
#   - `tr ' ' '\n'`  — bo'shliqni newline belgisiga almashtiradi
#   - `|` operatori bilan birinchi buyruq natijasini ikkinchisiga uzating
#
# --- English ---
# TASK: First pipeline
# LEVEL: ★★☆☆☆
# TOPIC: part1/03-pipes-redirection · | va tr
#
# Print the text "salom dunyo bash" with each word ON A SEPARATE LINE:
#     salom
#     dunyo
#     bash
#
# Hint:
#   - `tr ' ' '\n'`  — replaces a space with a newline character
#   - with the `|` operator, send the output of the first command into the second

# I AM NOT DONE

# TODO: echo + pipe + tr ishlatib har so'zni yangi qatorga chiqaring

# === TEST META ===
# @test:stdout-cmd: printf 'salom\ndunyo\nbash\n'
# @test:exit: 0
