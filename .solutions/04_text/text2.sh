#!/usr/bin/env bash
# SOLUTION: text2 — grep -ic
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
