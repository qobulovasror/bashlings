#!/usr/bin/env bash
#
# MASHQ: Moslashlar sonini sanash
# DARAJA: ★★★☆☆
# MAVZU: part1/03-pipes-redirection · grep -c
#
# log.txt fayli ichida bir nechta INFO va ERROR satr bor.
# Faqat "ERROR" satrlarning SONINI chiqaring.
#
# Maslahat:
#   - `grep -c <pattern> <fayl>`   — pattern qancha qatorda uchragani (faqat son)
#   - Yoki: `grep <pattern> <fayl> | wc -l` — lekin macOS'da wc'da leading whitespace bor

# I AM NOT DONE

work=/tmp/bashlings-pipe5
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
cat > log.txt <<EOF
[INFO] start
[ERROR] disk full
[INFO] reading
[ERROR] timeout
[INFO] done
EOF

# TODO: log.txt'da nechta "ERROR" qatori borligini sanang

# === TEST META ===
# @test:stdout: 2
# @test:exit: 0
