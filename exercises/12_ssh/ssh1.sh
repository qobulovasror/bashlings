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
#
# --- English ---
# TASK: SSH connection command
# LEVEL: ★☆☆☆☆
# TOPIC: part3/02-ssh · ssh -p user@host
#
# From the USER, HOST and PORT variables below, print the full ssh
# command:
#
#     ssh -p 2222 deploy@example.com
#
# Hint:
#   - The default SSH port is 22; a non-standard port is given with `-p N`
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
