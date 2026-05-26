#!/usr/bin/env bash
# SOLUTION: ssh3 — ssh-keygen ed25519 kalit
EMAIL="ali@example.com"
PATH_KEY="~/.ssh/id_ed25519"
echo "ssh-keygen -t ed25519 -C \"$EMAIL\" -f $PATH_KEY"

# === TEST META ===
# @test:stdout: ssh-keygen -t ed25519 -C "ali@example.com" -f ~/.ssh/id_ed25519
# @test:exit: 0
