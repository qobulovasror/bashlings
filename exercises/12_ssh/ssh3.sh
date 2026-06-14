#!/usr/bin/env bash
#
# MASHQ: ssh-keygen — ed25519 kalit yaratish
# DARAJA: ★★☆☆☆
# MAVZU: part3/02-ssh · ssh-keygen -t ed25519
#
# Zamonaviy va xavfsiz kalit yaratish komandasini chiqaring (3 ta flag):
#
#     ssh-keygen -t ed25519 -C "ali@example.com" -f ~/.ssh/id_ed25519
#
# Maslahat:
#   -t ed25519  → algoritm (RSA emas, ED25519 — qisqa va kuchli)
#   -C "..."    → kommentariya (email odatda)
#   -f <path>   → kalitni qaysi faylga saqlash
#
# --- English ---
# TASK: ssh-keygen — create an ed25519 key
# LEVEL: ★★☆☆☆
# TOPIC: part3/02-ssh · ssh-keygen -t ed25519
#
# Print the command to create a modern, secure key (3 flags):
#
#     ssh-keygen -t ed25519 -C "ali@example.com" -f ~/.ssh/id_ed25519
#
# Hint:
#   -t ed25519  → algorithm (not RSA, ED25519 — short and strong)
#   -C "..."    → comment (usually an email)
#   -f <path>   → which file to save the key to

# I AM NOT DONE

EMAIL="ali@example.com"
PATH_KEY="~/.ssh/id_ed25519"

# TODO: ssh-keygen komandasini chiqaring
echo "ssh-keygen"

# === TEST META ===
# @test:stdout: ssh-keygen -t ed25519 -C "ali@example.com" -f ~/.ssh/id_ed25519
# @test:exit: 0
