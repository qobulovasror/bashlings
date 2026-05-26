#!/usr/bin/env bash
# SOLUTION: cicd6 — conditional step (if:)
cat <<'EOF'
- name: Deploy to production
  if: github.ref == 'refs/heads/main'
  run: ./deploy.sh
EOF

# === TEST META ===
# @test:stdout-cmd: printf "%s\n" "- name: Deploy to production" "  if: github.ref == 'refs/heads/main'" "  run: ./deploy.sh"
# @test:exit: 0
