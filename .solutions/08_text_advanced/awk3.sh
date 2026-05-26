#!/usr/bin/env bash
# SOLUTION: awk3
work=/tmp/bashlings-awk3
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
cat > log.txt <<EOF
INFO start
ERROR disk
INFO retry
ERROR timeout
ERROR conn
EOF

awk '{count[$1]++} END {for (k in count) print k, count[k]}' log.txt | sort

# === TEST META ===
# @test:stdout-cmd: printf 'ERROR 3\nINFO 2\n'
# @test:exit: 0
