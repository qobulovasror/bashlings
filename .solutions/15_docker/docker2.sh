#!/usr/bin/env bash
# SOLUTION: docker2 — oddiy Dockerfile (heredoc)
cat <<EOF
FROM python:3.11-alpine
WORKDIR /app
COPY app.py .
CMD ["python", "app.py"]
EOF

# === TEST META ===
# @test:stdout-cmd: printf 'FROM python:3.11-alpine\nWORKDIR /app\nCOPY app.py .\nCMD ["python", "app.py"]\n'
# @test:exit: 0
