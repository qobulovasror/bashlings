#!/usr/bin/env bash
# SOLUTION: ssh1 — SSH ulanish komandasi
USER="deploy"
HOST="example.com"
PORT=2222
echo "ssh -p $PORT $USER@$HOST"

# === TEST META ===
# @test:stdout: ssh -p 2222 deploy@example.com
# @test:exit: 0
