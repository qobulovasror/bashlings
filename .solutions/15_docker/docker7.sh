#!/usr/bin/env bash
# SOLUTION: docker7 — docker-compose.yml
cat <<EOF
services:
  web:
    image: nginx:alpine
    ports:
      - "8080:80"
    environment:
      ENV: production
EOF

# === TEST META ===
# @test:stdout-cmd: printf 'services:\n  web:\n    image: nginx:alpine\n    ports:\n      - "8080:80"\n    environment:\n      ENV: production\n'
# @test:exit: 0
