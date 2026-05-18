#!/usr/bin/env bash
#
# MASHQ: Case-insensitive qidiruv
# DARAJA: ★★★☆☆
# MAVZU: part1/04-text-processing · grep -i, grep -c
#
# log.txt da turli registrli "error" so'zlari bor:
#     [ERROR], [error], [Error]  — uchchalasi ham
# Bularni HAMMA REGISTRDA topib, SONI ni chiqaring.
#
# Maslahat:
#   - `grep -i` — case-insensitive (registrga e'tibor bermaydi)
#   - `grep -c` — moslashlar sonini chiqaradi
#   - Ikki flagni birlashtirib ishlatish mumkin: `grep -ic` yoki `grep -i -c`

# I AM NOT DONE

work=/tmp/bashlings-text2
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
cat > log.txt <<EOF
[info] system ok
[ERROR] disk full
[error] timeout
[INFO] retry
[Error] connection lost
EOF

# TODO: log.txt'da "error" so'zining barcha registrlardagi sonini chiqaring

# === TEST META ===
# @test:stdout: 3
# @test:exit: 0
