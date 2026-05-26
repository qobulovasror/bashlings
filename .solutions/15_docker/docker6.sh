#!/usr/bin/env bash
# SOLUTION: docker6 — multi-stage Dockerfile
cat <<EOF
FROM golang:1.22 AS build
WORKDIR /src
COPY . .
RUN go build -o app

FROM alpine:3.20
COPY --from=build /src/app /app
CMD ["/app"]
EOF

# === TEST META ===
# @test:stdout-cmd: printf 'FROM golang:1.22 AS build\nWORKDIR /src\nCOPY . .\nRUN go build -o app\n\nFROM alpine:3.20\nCOPY --from=build /src/app /app\nCMD ["/app"]\n'
# @test:exit: 0
