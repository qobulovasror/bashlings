#!/usr/bin/env bash
#
# MASHQ: SSH local port forwarding
# DARAJA: ★★★☆☆
# MAVZU: part3/02-ssh · ssh -L
#
# Bastion server orqali ichki tarmoqdagi DB ga lokal portdan ulanish uchun
# komandani chiqaring:
#
#     ssh -L 5433:db.internal:5432 user@bastion.io
#
# `-L LOCAL:DEST_HOST:DEST_PORT` formati:
#   LOCAL      = sizning kompyuteringizdagi port (5433)
#   DEST_HOST  = bastion ichidan ko'rinadigan host (db.internal)
#   DEST_PORT  = o'sha host'dagi port (5432)
#
# Natijada `psql -h localhost -p 5433` ichki DB'ga ulanadi.
#
# --- English ---
# TASK: SSH local port forwarding
# LEVEL: ★★★☆☆
# TOPIC: part3/02-ssh · ssh -L
#
# Print the command to connect from a local port to a DB on the internal
# network through a bastion server:
#
#     ssh -L 5433:db.internal:5432 user@bastion.io
#
# `-L LOCAL:DEST_HOST:DEST_PORT` format:
#   LOCAL      = the port on your computer (5433)
#   DEST_HOST  = the host visible from inside the bastion (db.internal)
#   DEST_PORT  = the port on that host (5432)
#
# As a result, `psql -h localhost -p 5433` connects to the internal DB.

# I AM NOT DONE

LOCAL_PORT=5433
REMOTE_HOST="db.internal"
REMOTE_PORT=5432
SSH_USER="user"
BASTION="bastion.io"

# TODO: yuqoridagi komandani chiqaring
echo "ssh -L"

# === TEST META ===
# @test:stdout: ssh -L 5433:db.internal:5432 user@bastion.io
# @test:exit: 0
