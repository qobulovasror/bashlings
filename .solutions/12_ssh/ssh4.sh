#!/usr/bin/env bash
# SOLUTION: ssh4 — ~/.ssh/config bloki (heredoc)
HOST_ALIAS="prod"
HOSTNAME="192.168.1.100"
USER="deploy"
PORT=2222
KEY="~/.ssh/id_ed25519"

cat <<EOF
Host $HOST_ALIAS
    HostName $HOSTNAME
    User $USER
    Port $PORT
    IdentityFile $KEY
EOF

# === TEST META ===
# @test:stdout-cmd: printf 'Host prod\n    HostName 192.168.1.100\n    User deploy\n    Port 2222\n    IdentityFile ~/.ssh/id_ed25519\n'
# @test:exit: 0
