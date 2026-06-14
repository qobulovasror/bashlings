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
#
# --- English ---
# TASK: Write to both sides
# LEVEL: ★★★☆☆
# TOPIC: part1/03-pipes-redirection · tee
#
# Write the text "salom" BOTH to the screen (stdout) AND to the file out.txt.
#
# Hint: `tee` is made exactly for this task.
#   echo "text" | tee fayl.txt
#
# The name tee comes from splitting water into two directions through a T-shaped pipe 🚰

# I AM NOT DONE

work=/tmp/bashlings-pipe7
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1

# TODO: echo + pipe + tee orqali out.txt'ga yozing va stdout'ga ham chiqaring

# === TEST META ===
# @test:stdout: salom
# @test:exit: 0
