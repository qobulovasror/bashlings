#!/usr/bin/env bash
#
# MASHQ: rsync sinxronlash komandasi
# DARAJA: ★★★☆☆
# MAVZU: part3/02-ssh · rsync -avz --delete
#
# Lokal `./dist/` papkasini serverga sinxronlash uchun ENG ko'p ishlatiladigan
# rsync kombinatsiyasini chiqaring:
#
#     rsync -avz --delete ./dist/ deploy@server.io:/var/www/app/
#
# Flag'lar:
#   -a       → archive rejim (recursive + permissions + timestamps)
#   -v       → verbose
#   -z       → kanalda siqib yuborish (tez tarmoq uchun)
#   --delete → manbada yo'q fayllarni TARGET'dan ham o'chir (haqiqiy sync)
#
# Diqqat: `./dist/` oxiridagi `/` — RSYNC uchun MUHIM:
#   ./dist  → "dist" papkasini O'zini ham nusxalash
#   ./dist/ → "dist" ICHIDAGI fayllarni nusxalash
#
# --- English ---
# TASK: rsync synchronization command
# LEVEL: ★★★☆☆
# TOPIC: part3/02-ssh · rsync -avz --delete
#
# Print the MOST commonly used rsync combination to sync the local
# `./dist/` directory to the server:
#
#     rsync -avz --delete ./dist/ deploy@server.io:/var/www/app/
#
# Flags:
#   -a       → archive mode (recursive + permissions + timestamps)
#   -v       → verbose
#   -z       → compress over the channel (for fast networks)
#   --delete → also delete files from TARGET that are gone from the source (true sync)
#
# Note: the trailing `/` in `./dist/` is IMPORTANT for RSYNC:
#   ./dist  → copy the "dist" directory itself too
#   ./dist/ → copy the files INSIDE "dist"

# I AM NOT DONE

SRC="./dist/"
USER="deploy"
HOST="server.io"
DEST="/var/www/app/"

# TODO: yuqoridagi komandani chiqaring
echo "rsync"

# === TEST META ===
# @test:stdout: rsync -avz --delete ./dist/ deploy@server.io:/var/www/app/
# @test:exit: 0
