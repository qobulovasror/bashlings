#!/usr/bin/env bash
#
# MASHQ: sed bilan qator o'chirish
# DARAJA: ★★★☆☆
# MAVZU: part2/03-sed-awk · Nd buyrug'i
#
# log.txt'da 5 ta qator (qator-1..5).
# 2 va 4-qatorlarni o'chirib, qolganini chiqaring:
#     qator-1
#     qator-3
#     qator-5
#
# Maslahat:
#   - `sed 'Nd'` — N-qatorni o'chirish
#   - Bir nechta: `sed '2d;4d'` yoki `sed -e '2d' -e '4d'`
#
# --- English ---
# TASK: Deleting lines with sed
# LEVEL: ★★★☆☆
# TOPIC: part2/03-sed-awk · the Nd command
#
# log.txt has 5 lines (qator-1..5).
# Delete lines 2 and 4 and print the rest:
#     qator-1
#     qator-3
#     qator-5
#
# Hint:
#   - `sed 'Nd'` — delete line N
#   - Multiple: `sed '2d;4d'` or `sed -e '2d' -e '4d'`

# I AM NOT DONE

work=/tmp/bashlings-sed2
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
printf 'qator-1\nqator-2\nqator-3\nqator-4\nqator-5\n' > log.txt

# TODO: 2 va 4 qatorlarni sed bilan o'chiring

# === TEST META ===
# @test:stdout-cmd: printf 'qator-1\nqator-3\nqator-5\n'
# @test:exit: 0
