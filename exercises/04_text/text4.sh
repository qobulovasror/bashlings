#!/usr/bin/env bash
#
# MASHQ: CSV ustunni ajratib olish
# DARAJA: ★★★☆☆
# MAVZU: part1/04-text-processing · cut -d -f
#
# users.csv tarkibida 3 ustun bor:
#     ali,25,toshkent
#     vali,30,samarqand
#     gulnora,28,buxoro
#
# Faqat BIRINCHI ustun (ismlar) ni chiqaring.
#
# Maslahat:
#   - `cut -d <delimiter> -f <field_number> <fayl>`
#   - `-d','` — vergul bilan ajratish
#   - `-f1`   — 1-field

# I AM NOT DONE

work=/tmp/bashlings-text4
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
cat > users.csv <<EOF
ali,25,toshkent
vali,30,samarqand
gulnora,28,buxoro
EOF

# TODO: users.csv'dan 1-ustunni (ismlarni) chiqaring

# === TEST META ===
# @test:stdout-cmd: printf 'ali\nvali\ngulnora\n'
# @test:exit: 0
