#!/usr/bin/env bash
#
# MASHQ: Counter pattern — log darajalari statistikasi
# DARAJA: ★★★★☆
# MAVZU: part2/03-sed-awk · count[$1]++
#
# log.txt'da turli darajadagi log qatorlari bor.
# Har darajaning sonini sanab, ALFAVIT tartibida chiqaring:
#     ERROR 3
#     INFO 2
#
# Maslahat:
#   - `awk '{count[$1]++} END {for (k in count) print k, count[k]}'`
#   - awk associative array order non-deterministic — sort orqali tartiblang
#   - Pipeline: `awk ... | sort`

# I AM NOT DONE

work=/tmp/bashlings-awk3
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
cat > log.txt <<EOF
INFO start
ERROR disk
INFO retry
ERROR timeout
ERROR conn
EOF

# TODO: counter pattern + sort

# === TEST META ===
# @test:stdout-cmd: printf 'ERROR 3\nINFO 2\n'
# @test:exit: 0
