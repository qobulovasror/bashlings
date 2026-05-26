#!/usr/bin/env bash
#
# MASHQ: JSON POST so'rov
# DARAJA: ★★★☆☆
# MAVZU: part3/01-network · curl -X POST + -H + -d
#
# API'ga JSON body yuborish uchun 3 ta flag birga kerak:
#   -X POST                                   → metod
#   -H 'Content-Type: application/json'       → header
#   -d '<JSON>'                               → body
#
# Quyidagi BODY va URL uchun to'liq komandani chiqaring (aynan shu tartibda):
#     curl -X POST -H 'Content-Type: application/json' -d '{"city":"Toshkent"}' https://httpbin.org/post
#
# Maslahat:
#   - JSON ichida " ishlatiladi, shuning uchun butun -d argumenti '...'
#     (bir tirnoq) ichida bo'ladi — bash o'zgartirib yubormaydi.
#   - Bash'da bir tirnoq ichidan o'zgaruvchi ishlatilmaydi; literal string
#     printf yoki echo orqali chiqariladi.

# I AM NOT DONE

URL="https://httpbin.org/post"
BODY='{"city":"Toshkent"}'

# TODO: yuqoridagi komandani chiqaring
echo "curl $URL"

# === TEST META ===
# @test:stdout: curl -X POST -H 'Content-Type: application/json' -d '{"city":"Toshkent"}' https://httpbin.org/post
# @test:exit: 0
