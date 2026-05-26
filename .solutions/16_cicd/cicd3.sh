#!/usr/bin/env bash
# SOLUTION: cicd3 — actions/checkout step
cat <<EOF
- name: Checkout code
  uses: actions/checkout@v4
EOF

# === TEST META ===
# @test:stdout-cmd: printf -- '- name: Checkout code\n  uses: actions/checkout@v4\n'
# @test:exit: 0
