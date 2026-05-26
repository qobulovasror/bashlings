#!/usr/bin/env bash
#
# MASHQ: Job va step bloki
# DARAJA: ★★☆☆☆
# MAVZU: part3/06-cicd · jobs/runs-on/steps
#
# Workflow ichidagi `jobs:` blokini chiqaring:
#
#     jobs:
#       test:
#         runs-on: ubuntu-latest
#         steps:
#           - name: Run tests
#             run: npm test
#
# Diqqat:
#   - YAML chekinishi: 2-spaces
#   - `- name:` — list element (dash + bo'sh joy)
#   - `run:` — shell buyrug'i
#
# Maslahat: heredoc orqali, chekinishni AYNAN saqlang.

# I AM NOT DONE

# TODO: yuqoridagi YAML'ni chiqaring
echo "jobs:"

# === TEST META ===
# @test:stdout-cmd: printf 'jobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n      - name: Run tests\n        run: npm test\n'
# @test:exit: 0
