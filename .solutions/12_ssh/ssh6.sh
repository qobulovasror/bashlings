#!/usr/bin/env bash
# SOLUTION: ssh6 — SSH local port forwarding
LOCAL_PORT=5433
REMOTE_HOST="db.internal"
REMOTE_PORT=5432
SSH_USER="user"
BASTION="bastion.io"
echo "ssh -L $LOCAL_PORT:$REMOTE_HOST:$REMOTE_PORT $SSH_USER@$BASTION"

# === TEST META ===
# @test:stdout: ssh -L 5433:db.internal:5432 user@bastion.io
# @test:exit: 0
