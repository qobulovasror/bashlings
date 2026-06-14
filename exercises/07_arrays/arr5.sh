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
#
# --- English ---
# TASK: Associative array (dictionary)
# LEVEL: ★★★★☆
# TOPIC: part2/02-arrays · declare -A
#
# Create and fill a `user` associative array:
#   user[name]="Ali"
#   user[city]="Toshkent"
#
# Then print "Ali Toshkent" (separated by a space).
#
# ⚠ macOS stock bash 3.2 — `declare -A` does not work.
# Run `brew install bash` and `#!/usr/bin/env bash` will find the newer one from PATH.
#
# Hint:
#   - `declare -A user` — declare it
#   - `user[key]="value"` — set an element
#   - `${user[key]}` — get a value

# I AM NOT DONE

# TODO: declare -A user


# TODO: user[name] va user[city] ni o'rnating


echo "${user[name]} ${user[city]}"

# === TEST META ===
# @test:stdout: Ali Toshkent
# @test:exit: 0
