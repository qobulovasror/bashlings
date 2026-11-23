#!/usr/bin/env bash
# SOLUTION: pipe5 — grep -c
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
