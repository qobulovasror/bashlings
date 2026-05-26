# 💡 docker6

## 1-bosqich
Multi-stage = 2 (yoki ko'p) `FROM` bloki bir Dockerfile ichida.

Birinchi blok build environment (toolchain), ikkinchi blok — runtime
(faqat compiled binary).

## 2-bosqich
Asosiy direktivalar:
- `FROM image AS name`         — stage'ga nom berish
- `COPY --from=name SRC DST`   — boshqa stage'dan fayl ko'chirish

## 3-bosqich
Heredoc orqali chiqarish:
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
