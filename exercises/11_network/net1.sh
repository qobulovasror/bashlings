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

# I AM NOT DONE

URL="https://api.github.com"

eko "curl XXX $URL"

# === TEST META ===
# @test:stdout: curl -fsSL https://api.github.com
# @test:exit: 0
