#!/usr/bin/env bash
# SOLUTION: text2 — grep -ic
work=/tmp/bashlings-text2
rm -rf "$work" && mkdir -p "$work" && cd "$work" || exit 1
cat > log.txt <<EOF
[info] system ok
[ERROR] disk full
[error] timeout
[INFO] retry
[Error] connection lost
EOF

grep -ic "error" log.txt

# === TEST META ===
# @test:stdout: 3
# @test:exit: 0
