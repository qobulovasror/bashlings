#!/usr/bin/env bash
#
# MASHQ: SSH auth.log'dan kirgan foydalanuvchilarni topish
# DARAJA: ★★★★☆
# MAVZU: part3/02-ssh · log parsing (awk)
#
# Quyida `/var/log/auth.log` ko'rinishidagi mock log berilgan.
# Faqat "Accepted publickey" muvaffaqiyatli kirishlardan FOYDALANUVCHI
# nomlarini chiqaring — har biri alohida qatorda, takrorlanmasdan.
#
# Kutilgan (alifbo tartibida):
#     ali
#     deploy
#     root
#
# Maslahat:
#   - `grep "Accepted publickey"` — faqat muvaffaqiyatli kirishlar
#   - `awk '{print $9}'` — "for USER from IP" — USER 9-ustun
#   - `sort -u` — alifbo bo'yicha tartiblash + takrorni olib tashlash
#
# --- English ---
# TASK: Find logged-in users from the SSH auth.log
# LEVEL: ★★★★☆
# TOPIC: part3/02-ssh · log parsing (awk)
#
# Below is a mock log in the form of `/var/log/auth.log`.
# Print only the USER names from the successful "Accepted publickey"
# logins — each on its own line, without duplicates.
#
# Expected (in alphabetical order):
#     ali
#     deploy
#     root
#
# Hint:
#   - `grep "Accepted publickey"` — only the successful logins
#   - `awk '{print $9}'` — "for USER from IP" — USER is the 9th column
#   - `sort -u` — sort alphabetically + remove duplicates

# I AM NOT DONE

log='May 26 08:14:01 host sshd[1234]: Accepted publickey for deploy from 10.0.0.5 port 51200 ssh2
May 26 08:14:05 host sshd[1235]: Failed password for invalid user admin from 10.0.0.7
May 26 08:15:11 host sshd[1240]: Accepted publickey for root from 10.0.0.5 port 51210 ssh2
May 26 08:16:02 host sshd[1245]: Accepted publickey for ali from 10.0.0.5 port 51220 ssh2
May 26 08:16:30 host sshd[1250]: Failed password for root from 10.0.0.9
May 26 08:17:00 host sshd[1255]: Accepted publickey for deploy from 10.0.0.5 port 51230 ssh2'

# TODO: muvaffaqiyatli foydalanuvchilarni unique va sorted ko'rinishda chiqaring
echo "$log"

# === TEST META ===
# @test:stdout-cmd: printf 'ali\ndeploy\nroot\n'
# @test:exit: 0
