#!/usr/bin/env bash
#
# MASHQ: Conditional step — faqat main branch'da
# DARAJA: ★★★☆☆
# MAVZU: part3/06-cicd · if: shartlar
#
# Step'lar shartli ishga tushirilishi mumkin. Quyidagi blokni chiqaring:
#
#     - name: Deploy to production
#       if: github.ref == 'refs/heads/main'
#       run: ./deploy.sh
#
# Eslatma: `github.ref` — push qilingan branch'ning to'liq ref'i.
# `refs/heads/main` — main branch'iga mos.
#
# Maslahat: `${{ ... }}` o'rab olish if: uchun kerak emas (ifoda boolean).
#
# --- English ---
# TASK: A conditional step — only on the main branch
# LEVEL: ★★★☆☆
# TOPIC: part3/06-cicd · if: conditions
#
# Steps can be run conditionally. Print the following block:
#
#     - name: Deploy to production
#       if: github.ref == 'refs/heads/main'
#       run: ./deploy.sh
#
# Note: `github.ref` — the full ref of the pushed branch.
# `refs/heads/main` — matches the main branch.
#
# Hint: wrapping in `${{ ... }}` is not needed for if: (the expression is boolean).

# I AM NOT DONE

# TODO: yuqoridagi step blokini chiqaring
echo "- name: ..."

# === TEST META ===
# @test:stdout-cmd: printf "%s\n" "- name: Deploy to production" "  if: github.ref == 'refs/heads/main'" "  run: ./deploy.sh"
# @test:exit: 0
