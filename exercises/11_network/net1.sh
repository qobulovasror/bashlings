#!/usr/bin/env bash
#
# MASHQ: curl standart flag kombinatsiyasi
# DARAJA: ★☆☆☆☆
# MAVZU: part3/01-network · curl -fsSL
#
# Skriptlar uchun curl'ning ENG standart flag kombinatsiyasi — `-fsSL`.
# Quyidagi URL uchun bu komandani sof matn ko'rinishida chiqaring:
#
#     curl -fsSL https://api.github.com
#
# Hozir `eko` xato — bash bunday buyruqni bilmaydi.
# Tuzating va `# I AM NOT DONE` qatorini o'chiring.
#
# Maslahat:
#   -f → 4xx/5xx da xato qaytar (server response'ni body qilib qaytarmasin)
#   -s → progress bar'ni o'chir
#   -S → -s ga qaramay haqiqiy xato chiqsin (silent yo'q)
#   -L → redirect'larni kuzat
#
# --- English ---
# TASK: The standard curl flag combination
# LEVEL: ★☆☆☆☆
# TOPIC: part3/01-network · curl -fsSL
#
# For scripts, the MOST standard curl flag combination is `-fsSL`.
# Print this command for the URL below as plain text:
#
#     curl -fsSL https://api.github.com
#
# Right now `eko` is wrong — bash does not know such a command.
# Fix it and remove the `# I AM NOT DONE` line.
#
# Hint:
#   -f → return an error on 4xx/5xx (don't return the server response as the body)
#   -s → turn off the progress bar
#   -S → still show a real error despite -s (not silent)
#   -L → follow redirects

# I AM NOT DONE

URL="https://api.github.com"

eko "curl XXX $URL"

# === TEST META ===
# @test:stdout: curl -fsSL https://api.github.com
# @test:exit: 0
