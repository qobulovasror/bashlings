#!/usr/bin/env bash
# SOLUTION: ssh2 — scp bilan fayl yuklash
SRC="./backup.tar.gz"
USER="deploy"
HOST="server.io"
DEST="/srv/backups/"
echo "scp $SRC $USER@$HOST:$DEST"

# === TEST META ===
# @test:stdout: scp ./backup.tar.gz deploy@server.io:/srv/backups/
# @test:exit: 0
