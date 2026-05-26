#!/usr/bin/env bash
#
# MASHQ: Foydalanuvchi nomi
# DARAJA: ★☆☆☆☆
# MAVZU: part1/01-introduction · whoami / $USER
#
# Joriy foydalanuvchining tizimdagi nomini chiqaring.
# Sizning kompyuteringizdagi foydalanuvchi nomi nima bo'lsa — shu chiqishi kerak.
#
# Maslahat: bu uchun bitta buyruq bor (whoami) yoki maxsus o'zgaruvchi ($USER).

echo $USER

# === TEST META ===
# @test:stdout-cmd: whoami
# @test:exit: 0
