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

# I AM NOT DONE

EMAIL="ali@example.com"
PATH_KEY="~/.ssh/id_ed25519"

# TODO: ssh-keygen komandasini chiqaring
echo "ssh-keygen"

# === TEST META ===
# @test:stdout: ssh-keygen -t ed25519 -C "ali@example.com" -f ~/.ssh/id_ed25519
# @test:exit: 0
