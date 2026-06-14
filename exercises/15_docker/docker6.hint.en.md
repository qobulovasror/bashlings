# 💡 docker6

## Step 1
Multi-stage = 2 (or more) `FROM` blocks inside one Dockerfile.

The first block is the build environment (toolchain), the second block is the
runtime (only the compiled binary).

## Step 2
Main directives:
- `FROM image AS name`         — give a name to the stage
- `COPY --from=name SRC DST`   — copy a file from another stage

## Step 3
Printing via heredoc:
```bash
cat <<EOF
FROM golang:1.22 AS build
WORKDIR /src
COPY . .
RUN go build -o app

FROM alpine:3.20
COPY --from=build /src/app /app
CMD ["/app"]
EOF
```
