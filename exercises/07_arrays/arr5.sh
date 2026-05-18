#!/usr/bin/env bash
#
# MASHQ: Associative array (lug'at)
# DARAJA: ★★★★☆
# MAVZU: part2/02-arrays · declare -A
#
# `user` associative array yarating va to'ldiring:
#   user[name]="Ali"
#   user[city]="Toshkent"
#
# So'ngra "Ali Toshkent" deb chiqaring (probel bilan).
#
# ⚠ macOS stock bash 3.2 — `declare -A` ishlamaydi.
# `brew install bash` qiling va `#!/usr/bin/env bash` PATH'dan yangini topadi.
#
# Maslahat:
#   - `declare -A user` — e'lon qilish
#   - `user[key]="value"` — element o'rnatish
#   - `${user[key]}` — qiymat olish

# I AM NOT DONE

# TODO: declare -A user


# TODO: user[name] va user[city] ni o'rnating


echo "${user[name]} ${user[city]}"

# === TEST META ===
# @test:stdout: Ali Toshkent
# @test:exit: 0
