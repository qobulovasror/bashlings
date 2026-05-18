#!/usr/bin/env bash
#
# MASHQ: Faylga yozish va o'qish
# DARAJA: ★☆☆☆☆
# MAVZU: part1/03-pipes-redirection · > redirect
#
# "salom" so'zini hello.txt fayliga yozing, keyin uni `cat` orqali chiqaring.
#
# Maslahat:
#   - `echo "matn" > fayl.txt`  — faylga yozish (mavjud bo'lsa qayta yoziladi)
#   - `cat fayl.txt`            — fayl mazmunini chiqarish

# I AM NOT DONE

work=/tmp/bashlings-pipe1
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1

# TODO: faylga "salom" so'zini yozing va keyin uni chiqaring

# === TEST META ===
# @test:stdout: salom
# @test:exit: 0
