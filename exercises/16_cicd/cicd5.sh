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
#
# --- English ---
# TASK: Using secrets
# LEVEL: ★★★☆☆
# TOPIC: part3/06-cicd · secrets and env
#
# Secrets are accessed inside a workflow via `${{ secrets.NAME }}`.
# Print the following step block:
#
#     - name: Publish to npm
#       run: npm publish
#       env:
#         NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
#
# Attention:
#   - `env:` inside a step — visible only to that step
#   - the `${{ ... }}` expression is not read by the shell — it is read by the workflow
#
# Hint: via a heredoc, with EXACTLY this indentation.

# I AM NOT DONE

# TODO: yuqoridagi step blokini chiqaring
echo "- name: ..."

# === TEST META ===
# @test:stdout-cmd: printf -- '- name: Publish to npm\n  run: npm publish\n  env:\n    NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}\n'
# @test:exit: 0
