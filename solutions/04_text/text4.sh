#!/usr/bin/env bash
# SOLUTION: text4 — cut -d -f
work=/tmp/bashlings-text4
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
cat > users.csv <<EOF
ali,25,toshkent
vali,30,samarqand
gulnora,28,buxoro
EOF

cut -d ',' -f 1 users.csv

# === TEST META ===
# @test:stdout-cmd: printf 'ali\nvali\ngulnora\n'
# @test:exit: 0
