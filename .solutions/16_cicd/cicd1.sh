#!/usr/bin/env bash
# SOLUTION: cicd1 — minimal workflow header
cat <<EOF
name: CI
on: push
EOF

# === TEST META ===
# @test:stdout-cmd: printf 'name: CI\non: push\n'
# @test:exit: 0
