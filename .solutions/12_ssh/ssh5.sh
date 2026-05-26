#!/usr/bin/env bash
# SOLUTION: ssh5 — rsync sinxronlash komandasi
SRC="./dist/"
USER="deploy"
HOST="server.io"
DEST="/var/www/app/"
echo "rsync -avz --delete $SRC $USER@$HOST:$DEST"

# === TEST META ===
# @test:stdout: rsync -avz --delete ./dist/ deploy@server.io:/var/www/app/
# @test:exit: 0
