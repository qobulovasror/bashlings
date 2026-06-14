#!/usr/bin/env bash
#
# MASHQ: actions/checkout — kodni klon qilish
# DARAJA: ★★☆☆☆
# MAVZU: part3/06-cicd · uses: action
#
# Har GitHub Actions workflow'i kodni klon qilish bilan boshlanadi.
# Quyidagi step bloki MUHIM — uses + name:
#
#     - name: Checkout code
#       uses: actions/checkout@v4
#
# Eslatma: `uses:` — marketplace'dan action ishlatish.
# `@v4` — pinned versiya (mutable tag emas, releasega bog'langan).
#
# Maslahat: 2 ta qator, ikkinchisi 2 ta bo'sh joy bilan chekinadi (parent
# list element ostida).
#
# --- English ---
# TASK: actions/checkout — cloning the code
# LEVEL: ★★☆☆☆
# TOPIC: part3/06-cicd · uses: action
#
# Every GitHub Actions workflow starts by cloning the code.
# The following step block is IMPORTANT — uses + name:
#
#     - name: Checkout code
#       uses: actions/checkout@v4
#
# Note: `uses:` — uses an action from the marketplace.
# `@v4` — a pinned version (not a mutable tag, bound to a release).
#
# Hint: 2 lines, the second is indented with 2 spaces (under the parent
# list element).

# I AM NOT DONE

# TODO: yuqoridagi step blokini chiqaring
echo "- name: ..."

# === TEST META ===
# @test:stdout-cmd: printf -- '- name: Checkout code\n  uses: actions/checkout@v4\n'
# @test:exit: 0
