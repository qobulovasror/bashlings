#!/usr/bin/env bash
#
# MASHQ: SSH ulanish komandasi
# DARAJA: ★☆☆☆☆
# MAVZU: part3/02-ssh · ssh -p user@host
#
# Quyidagi USER, HOST va PORT o'zgaruvchilaridan to'liq ssh komandasini
# chiqaring:
#
#     ssh -p 2222 deploy@example.com
#
# Maslahat:
#   - SSH default port 22; standart bo'lmagan portni `-p N` bilan beriladi
#   - Format: ssh -p <port> <user>@<host>

# I AM NOT DONE

USER="deploy"
HOST="example.com"
PORT=2222

# TODO: yuqoridagi komandani chiqaring
echo "ssh $USER@$HOST"

# === TEST META ===
# @test:stdout: ssh -p 2222 deploy@example.com
# @test:exit: 0
