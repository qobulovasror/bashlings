#!/usr/bin/env bash
#
# SOLUTION: intro5 — Tizim ma'lumotlari

echo "Foydalanuvchi: $USER"
echo "Home: $HOME"
echo "Joriy: $PWD"

# === TEST META ===
# @test:stdout-cmd: printf 'Foydalanuvchi: %s\nHome: %s\nJoriy: %s\n' "$USER" "$HOME" "$PWD"
# @test:exit: 0
