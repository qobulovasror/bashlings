#!/usr/bin/env bash
#
# MASHQ: Ikkala tomonga yozish
# DARAJA: ★★★☆☆
# MAVZU: part1/03-pipes-redirection · tee
#
# "salom" matnini HAM ekranga (stdout) HAM out.txt fayliga yozing.
#
# Maslahat: `tee` aynan shu vazifa uchun yaratilgan.
#   echo "matn" | tee fayl.txt
#
# tee'ning ismi suvni T-shaped truba orqali ikki tomonga ajratishdan kelib chiqqan 🚰

# I AM NOT DONE

work=/tmp/bashlings-pipe7
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1

# TODO: echo + pipe + tee orqali out.txt'ga yozing va stdout'ga ham chiqaring

# === TEST META ===
# @test:stdout: salom
# @test:exit: 0
