#!/usr/bin/env bash
#
# MASHQ: ~/.ssh/config bloki
# DARAJA: ★★★☆☆
# MAVZU: part3/02-ssh · SSH config faylini avto-yaratish
#
# O'zgaruvchilardan ~/.ssh/config'ga qo'shiladigan blokni HEREDOC orqali
# chiqaring (aynan quyidagi formatda):
#
#     Host prod
#         HostName 192.168.1.100
#         User deploy
#         Port 2222
#         IdentityFile ~/.ssh/id_ed25519
#
# Diqqat: chekinish — har property qatori 4 ta bo'sh joy bilan boshlanadi.
#
# Maslahat:
#   - Heredoc: cat <<EOF ... EOF (interpolatsiya bo'ladi)
#   - cat <<'EOF' (tirnoq bilan) — interpolatsiya YO'Q
#   - Bizga interpolatsiya kerak → tirnoqsiz EOF
#
# --- English ---
# TASK: ~/.ssh/config block
# LEVEL: ★★★☆☆
# TOPIC: part3/02-ssh · auto-generate the SSH config file
#
# From the variables, print the block to be added to ~/.ssh/config using a
# HEREDOC (in exactly this format):
#
#     Host prod
#         HostName 192.168.1.100
#         User deploy
#         Port 2222
#         IdentityFile ~/.ssh/id_ed25519
#
# Note: indentation — each property line starts with 4 spaces.
#
# Hint:
#   - Heredoc: cat <<EOF ... EOF (interpolation happens)
#   - cat <<'EOF' (with quotes) — NO interpolation
#   - We need interpolation → unquoted EOF

# I AM NOT DONE

HOST_ALIAS="prod"
HOSTNAME="192.168.1.100"
USER="deploy"
PORT=2222
KEY="~/.ssh/id_ed25519"

# TODO: yuqoridagi formatda config blokini chiqaring
echo "Host $HOST_ALIAS"

# === TEST META ===
# @test:stdout-cmd: printf 'Host prod\n    HostName 192.168.1.100\n    User deploy\n    Port 2222\n    IdentityFile ~/.ssh/id_ed25519\n'
# @test:exit: 0
