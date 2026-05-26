#!/usr/bin/env bash
# SOLUTION: pipe5 — grep -c
work=/tmp/bashlings-pipe5
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
cat > log.txt <<EOF
[INFO] start
[ERROR] disk full
[INFO] reading
[ERROR] timeout
[INFO] done
EOF

grep -c "ERROR" log.txt

# === TEST META ===
# @test:stdout: 2
# @test:exit: 0
