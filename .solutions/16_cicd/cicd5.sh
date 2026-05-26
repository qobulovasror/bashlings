#!/usr/bin/env bash
# SOLUTION: cicd5 — secret bilan step
cat <<'EOF'
- name: Publish to npm
  run: npm publish
  env:
    NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
EOF

# === TEST META ===
# @test:stdout-cmd: printf -- '- name: Publish to npm\n  run: npm publish\n  env:\n    NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}\n'
# @test:exit: 0
