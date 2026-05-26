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
