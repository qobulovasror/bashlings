#!/usr/bin/env bash
#
# MASHQ: scp bilan fayl yuklash
# DARAJA: ★★☆☆☆
# MAVZU: part3/02-ssh · scp src user@host:dest
#
# Lokal `./backup.tar.gz` faylini serverga `/srv/backups/` papkasiga
# nusxalash uchun to'liq komandani chiqaring:
#
#     scp ./backup.tar.gz deploy@server.io:/srv/backups/
#
# Maslahat:
#   - scp sintaksisi: `scp SOURCE TARGET`
#   - Remote target ko'rinishi: `user@host:/path/`
#   - Manba lokal fayl yo'li bo'lganida prefix kerak emas
#
# --- English ---
# TASK: Upload a file with scp
# LEVEL: ★★☆☆☆
# TOPIC: part3/02-ssh · scp src user@host:dest
#
# Print the full command to copy the local `./backup.tar.gz` file to the
# `/srv/backups/` directory on the server:
#
#     scp ./backup.tar.gz deploy@server.io:/srv/backups/
#
# Hint:
#   - scp syntax: `scp SOURCE TARGET`
#   - Remote target form: `user@host:/path/`
#   - No prefix is needed when the source is a local file path

# I AM NOT DONE

SRC="./backup.tar.gz"
USER="deploy"
HOST="server.io"
DEST="/srv/backups/"

# TODO: scp komandasini chiqaring
echo "scp $SRC"

# === TEST META ===
# @test:stdout: scp ./backup.tar.gz deploy@server.io:/srv/backups/
# @test:exit: 0
