#!/usr/bin/env bash
# SOLUTION: cicd2 — job va step bloki
cat <<EOF
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - name: Run tests
        run: npm test
EOF

# === TEST META ===
# @test:stdout-cmd: printf 'jobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n      - name: Run tests\n        run: npm test\n'
# @test:exit: 0
