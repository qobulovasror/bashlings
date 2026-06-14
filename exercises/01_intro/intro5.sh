#!/usr/bin/env bash
#
# MASHQ: Tizim ma'lumotlari
# DARAJA: ★★☆☆☆
# MAVZU: part1/01-introduction · variables + echo composition
#
# Quyidagi 3 qatorni aniq shu tartibda chiqaring:
#
#     Foydalanuvchi: <username>
#     Home: <home katalog>
#     Joriy: <pwd>
#
# Har qator alohida `echo` bo'lsin. Maxsus o'zgaruvchilardan foydalaning:
#   $USER, $HOME, $PWD
#
# O'zgaruvchilarni qo'shtirnoq ichida ishlating: "...$USER..."
#
# --- English ---
# TASK: System information
# LEVEL: ★★☆☆☆
# TOPIC: part1/01-introduction · variables + echo composition
#
# Print the following 3 lines in exactly this order:
#
#     Foydalanuvchi: <username>
#     Home: <home directory>
#     Joriy: <pwd>
#
# Each line should be a separate `echo`. Use the special variables:
#   $USER, $HOME, $PWD
#
# Use the variables inside double quotes: "...$USER..."

# I AM NOT DONE

echo "TODO 1"
echo "TODO 2"
echo "TODO 3"

# === TEST META ===
# @test:stdout-cmd: printf 'Foydalanuvchi: %s\nHome: %s\nJoriy: %s\n' "$USER" "$HOME" "$PWD"
# @test:exit: 0
