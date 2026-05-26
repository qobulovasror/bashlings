#!/usr/bin/env bash
# SOLUTION: cicd4 — matrix strategy
cat <<'EOF'
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest]
runs-on: ${{ matrix.os }}
EOF

# === TEST META ===
# @test:stdout-cmd: printf 'strategy:\n  matrix:\n    os: [ubuntu-latest, macos-latest]\nruns-on: ${{ matrix.os }}\n'
# @test:exit: 0
