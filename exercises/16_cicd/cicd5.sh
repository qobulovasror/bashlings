#!/usr/bin/env bash
#
# MASHQ: Secret'lardan foydalanish
# DARAJA: ★★★☆☆
# MAVZU: part3/06-cicd · secrets va env
#
# Secret'lar workflow ichida `${{ secrets.NAME }}` orqali olinadi.
# Quyidagi step blokini chiqaring:
#
#     - name: Publish to npm
#       run: npm publish
#       env:
#         NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
#
# Diqqat:
#   - `env:` step ichida — faqat shu stepga ko'rinadi
#   - `${{ ... }}` ekspressiyasi shell'da emas — workflow'da o'qiladi
#
# Maslahat: heredoc orqali, AYNAN shu chekinish bilan.

# I AM NOT DONE

# TODO: yuqoridagi step blokini chiqaring
echo "- name: ..."

# === TEST META ===
# @test:stdout-cmd: printf -- '- name: Publish to npm\n  run: npm publish\n  env:\n    NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}\n'
# @test:exit: 0
