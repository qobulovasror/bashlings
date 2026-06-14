---
title: "Arrays and dictionaries"
description: "Indexed and associative arrays, iteration, IFS, parsing and real CSV/TODO examples."
---

# 2. Arrays and dictionaries

> **🎯 What you will learn in this chapter:**
> - **Indexed arrays** — `arr=(a b c)`, appending, removing, iterating
> - **Associative arrays** (dictionaries) — `declare -A` (Bash 4+)
> - `"${arr[@]}"` vs `"${arr[*]}"` — the critical difference
> - CSV and string parsing with **`IFS`**
> - A real example — a small TODO list manager
>
> **⏱ Time:** ~30 minutes
> **🧪 Exercises:** `bashlings watch` — 6 interactive exercises ready ([`exercises/07_arrays/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/07_arrays))

---

## 2.1. Why arrays?

Imagine your script deploys to three servers. The first approach:

```bash
servers="alpha beta gamma"
for s in $servers; do
    deploy "$s"
done
```

It works. But what if a server name contains a **space**? Like `"beta server"`? Bash splits it into 2 separate elements:

```bash
servers="alpha beta server gamma"
# deploy: alpha, beta, server, gamma — 4 of them!
```

An array, on the other hand, is a **proper data structure** — each element is stored in its own cell:

```bash
servers=("alpha" "beta server" "gamma")
for s in "${servers[@]}"; do
    deploy "$s"
done
# deploy: alpha, "beta server", gamma — 3 of them, correct ✓
```

::: tip Key idea
In Bash, whenever you need to store **multiple values** — use an array, not a string. It protects you from spaces, special characters and the word-splitting trap.
:::

---

## 2.2. Indexed arrays — declaring

Indexed arrays are the simplest type: elements are stored by **integer indices** (starting from 0).

### Syntax variants

```bash
# Empty array
arr=()

# With several elements
fruits=("olma" "anor" "uzum")

# Range (brace expansion)
nums=({1..10})            # 1 2 3 4 5 6 7 8 9 10

# With mixed-type values
mixed=("matn" 42 3.14 "yana matn")
```

### Defining with `declare -a`

```bash
declare -a cities
cities[0]="Toshkent"
cities[1]="Samarqand"
cities[2]="Buxoro"
```

`declare -a` — **explicitly marks** it as an indexed array in bash. Inside a function it is used as `local -a`.

### Indices start from 0

```bash
fruits=("olma" "anor" "uzum")
#         ↑0      ↑1     ↑2
```

::: warning Be careful with command output
```bash
files=($(ls *.txt))   # ⚠ DANGEROUS
```

The reason: if a file name contains a space, bash itself word-splits the output of `ls`. **`my file.txt`** → `"my"` and `"file.txt"` — it ends up as two elements.

The correct way (Bash 4+):
```bash
mapfile -t files < <(ls *.txt)
```
:::

---

## 2.3. Working with elements

### Accessing a single element

```bash
fruits=("olma" "anor" "uzum")

echo "${fruits[0]}"      # olma
echo "${fruits[1]}"      # anor
echo "${fruits[-1]}"     # uzum (last — Bash 4.2+)
```

⚠ **Note:** `$fruits` (without `${fruits}`, without an index) **returns the first element**, not all of them. This is a classic trap.

```bash
echo "$fruits"           # olma  ❌ many people think this means "all"
echo "${fruits[@]}"      # olma anor uzum  ✓
```

### Array length (element count)

```bash
echo "${#fruits[@]}"     # 3
```

### Adding an element (append)

```bash
fruits+=("shaftoli")     # appends to the end
echo "${fruits[@]}"      # olma anor uzum shaftoli
```

Adding several elements at once:

```bash
fruits+=("nok" "olcha")
```

### Changing an element

```bash
fruits[1]="banan"
echo "${fruits[@]}"      # olma banan uzum shaftoli ...
```

### Removing an element

```bash
unset 'fruits[1]'
echo "${fruits[@]}"      # olma uzum shaftoli ...
```

::: warning `unset` leaves a "live state"
`unset arr[1]` — removes the element, but **does not re-index** the array:

```bash
arr=(a b c d)
unset 'arr[1]'
echo "${arr[@]}"         # a c d  (b is gone)
echo "${arr[2]}"         # c       (index 2 is preserved)
echo "${arr[1]}"         # (empty — because it was removed)
```

To compact the indices you need to rewrite it:
```bash
arr=("${arr[@]}")
```
:::

### Removing the whole array

```bash
unset fruits             # the array disappears entirely
```

---

## 2.4. Iteration

### The most common — `for ... in`

```bash
fruits=("olma" "anor" "uzum")

for f in "${fruits[@]}"; do
    echo "Meva: $f"
done
```

::: danger Don't forget the quotes!
`for f in ${fruits[@]}` — without quotes — word-splits each element.
**Always** write it as `"${fruits[@]}"`.
:::

### With indices

When you need the element + its position:

```bash
fruits=("olma" "anor" "uzum")

for i in "${!fruits[@]}"; do
    echo "$i: ${fruits[$i]}"
done
# 0: olma
# 1: anor
# 2: uzum
```

`${!arr[@]}` — returns the **list of indices** (not the values).

### Slicing — partial cutting

```bash
nums=(10 20 30 40 50 60 70)

# ${arr[@]:start:count}
echo "${nums[@]:2:3}"    # 30 40 50  (3 elements starting from position 2)
echo "${nums[@]:4}"      # 50 60 70  (from position 4 to the end)
echo "${nums[@]: -2}"    # 60 70     (last 2 — a space is required!)
```

::: tip Negative slicing
`${nums[@]:-2}` — this is the **default-value operator**, not slicing!
A negative offset requires a **space**: `${nums[@]: -2}`.
:::

### With `while` (less common)

```bash
i=0
while [[ $i -lt ${#fruits[@]} ]]; do
    echo "${fruits[$i]}"
    ((i++))
done
```

---

## 2.5. `"${arr[@]}"` vs `"${arr[*]}"` — the critical difference

This is the most confusing aspect of bash. Both are understood as "all elements", **but inside quotes they behave completely differently**.

```bash
arr=("salom dunyo" "bash" "uz")

# @ — each element separately
for x in "${arr[@]}"; do echo "[$x]"; done
# [salom dunyo]
# [bash]
# [uz]

# * — a single string, joined by the IFS character (default IFS = space)
for x in "${arr[*]}"; do echo "[$x]"; done
# [salom dunyo bash uz]   ← a single string!
```

| Syntax         | Result inside quotes                       |
|----------------|--------------------------------------------|
| `"${arr[@]}"`  | **Each element — a separate argument**     |
| `"${arr[*]}"`  | **A single string, joined with IFS**       |
| `${arr[@]}`    | Unquoted — risk of word-splitting          |

::: tip Rule
**In 99% of cases** use `"${arr[@]}"`. Use `"${arr[*]}"` only when you need to **turn it into a string**.
:::

### Custom separator with `IFS`

`${arr[*]}` uses the IFS character:

```bash
arr=("a" "b" "c")

IFS=, echo "${arr[*]}"
# a,b,c

IFS='|' echo "${arr[*]}"
# a|b|c
```

This is essentially the technique for turning a list into a CSV line.

---

## 2.6. Associative arrays — dictionaries (Bash 4+)

An associative array — stores by **string keys**. In other languages this is called a "dictionary", "map" or "hashtable".

::: danger The macOS problem
On macOS the default bash is **3.2** (from 2007). It **does not support associative arrays**.

The fix:
```bash
brew install bash
which bash               # /opt/homebrew/bin/bash
```

`#!/usr/bin/env bash` at the start of a script — `env` finds the newer bash from PATH.

Checking the version:
```bash
bash --version           # should be 5.x
```
:::

### Declaring and filling

```bash
declare -A user

user[name]="Ali"
user[age]=25
user[city]="Toshkent"
```

Or all at once:

```bash
declare -A user=(
    [name]="Ali"
    [age]=25
    [city]="Toshkent"
)
```

### Accessing values

```bash
echo "${user[name]}"     # Ali
echo "${user[age]}"      # 25
```

A nonexistent key — returns an empty string (not an error):

```bash
echo "${user[phone]}"    # (empty)
```

### Keys and values

```bash
echo "${!user[@]}"       # name age city  (keys)
echo "${user[@]}"        # Ali 25 Toshkent (values)
echo "${#user[@]}"       # 3              (total)
```

::: warning Order is not guaranteed
In an associative array the order of keys depends on the internal structure of the **hash table**. The iteration order is not reliable. If you need order, store the keys in a separate indexed array.
:::

### Iteration

```bash
for key in "${!user[@]}"; do
    echo "$key = ${user[$key]}"
done
# name = Ali
# age = 25
# city = Toshkent
```

### Removing an element

```bash
unset 'user[city]'
```

### A real example — a counter for a set of files

```bash
#!/usr/bin/env bash
declare -A count

# Count the extensions of files in the current directory
for f in *; do
    ext="${f##*.}"
    ((count[$ext]++))
done

for ext in "${!count[@]}"; do
    printf '%-10s %d\n' "$ext" "${count[$ext]}"
done
```

Result:
```text
md         12
sh          8
txt         3
```

---

## 2.7. `IFS` and string parsing

**IFS** (Internal Field Separator) — bash's word-splitting character. Its default value: space + tab + newline.

### `read -ra` — splitting a string into an array

```bash
csv="ali,25,toshkent"

IFS=',' read -ra fields <<< "$csv"

echo "${fields[0]}"      # ali
echo "${fields[1]}"      # 25
echo "${fields[2]}"      # toshkent
```

`-r` — do not interpret backslashes (always use it).
`-a fields` — write the result into the `fields` array.
`<<<` — here-string (single-line stdin).

### Joining an array into a string

```bash
words=("salom" "dunyo" "bash")

# With spaces
result="${words[*]}"
echo "$result"           # salom dunyo bash

# With a custom separator
(IFS=','; result="${words[*]}"; echo "$result")
# salom,dunyo,bash
```

::: tip Be careful with `IFS` and subshells
We put `(IFS=','; ...)` in parentheses so that changing `IFS` stays only within this block. Outside, IFS does not change.
:::

### From a file into an array, line by line

```bash
# The cleanest way for Bash 4+
mapfile -t lines < /etc/passwd

echo "${#lines[@]}"      # number of lines
echo "${lines[0]}"       # first line
```

`mapfile -t` — makes each line a separate element, and `-t` strips the newline.

For older bash:
```bash
lines=()
while IFS= read -r line; do
    lines+=("$line")
done < /etc/passwd
```

---

## 2.8. A real example — a TODO list manager

A complete example demonstrating arrays in practice:

```bash
#!/usr/bin/env bash
#
# todo.sh — a small TODO list manager
#
# Usage:
#   todo.sh add "Buyruq"
#   todo.sh list
#   todo.sh done 2
#

set -euo pipefail

readonly TODO_FILE="${TODO_FILE:-$HOME/.todo}"

# Load the file (as an array)
load_tasks() {
    if [[ -f "$TODO_FILE" ]]; then
        mapfile -t tasks < "$TODO_FILE"
    else
        tasks=()
    fi
}

# Save the array to the file
save_tasks() {
    printf '%s\n' "${tasks[@]}" > "$TODO_FILE"
}

cmd_add() {
    local item="$*"
    [[ -z "$item" ]] && { echo "Foydalanish: todo add <matn>" >&2; exit 1; }
    tasks+=("$item")
    save_tasks
    echo "✓ Qo'shildi: $item"
}

cmd_list() {
    if [[ ${#tasks[@]} -eq 0 ]]; then
        echo "(ro'yxat bo'sh)"
        return
    fi
    for i in "${!tasks[@]}"; do
        printf '%2d. %s\n' "$((i + 1))" "${tasks[$i]}"
    done
}

cmd_done() {
    local n="$1"
    local idx=$((n - 1))
    if [[ -z "${tasks[$idx]:-}" ]]; then
        echo "❌ #$n topilmadi" >&2
        exit 1
    fi
    echo "✓ Bajarildi: ${tasks[$idx]}"
    unset 'tasks[idx]'
    tasks=("${tasks[@]}")    # re-index and compact
    save_tasks
}

# --- Main ---
load_tasks
case "${1:-list}" in
    add)  shift; cmd_add "$@" ;;
    list) cmd_list ;;
    done) cmd_done "$2" ;;
    *) echo "Foydalanish: $0 {add|list|done}"; exit 1 ;;
esac
```

Let's try it out:

```bash
$ todo.sh add "non sotib olish"
✓ Qo'shildi: non sotib olish

$ todo.sh add "kitobni o'qish"
✓ Qo'shildi: kitobni o'qish

$ todo.sh list
 1. non sotib olish
 2. kitobni o'qish

$ todo.sh done 1
✓ Bajarildi: non sotib olish

$ todo.sh list
 1. kitobni o'qish
```

### What was used in this example?

| Technique                           | Where                        |
|-------------------------------------|------------------------------|
| `mapfile -t` (file → array)          | `load_tasks`                 |
| `printf '%s\n' "${arr[@]}"`         | `save_tasks` (writing to file) |
| `tasks+=(...)` appending             | `cmd_add`                    |
| `"${!arr[@]}"` indices               | `cmd_list`                   |
| `unset 'arr[i]'` + recompacting     | `cmd_done`                   |
| `${var:-default}` empty-value check | `cmd_list`                   |

---

## 2.9. Common mistakes

::: danger Classic traps

1. **`$arr` — only the first element.**
   ```bash
   arr=(a b c)
   echo "$arr"      # a   ❌
   echo "${arr[@]}" # a b c  ✓
   ```

2. **Unquoted iteration.**
   ```bash
   for x in ${arr[@]}; do ...   # ❌ word-splitting
   for x in "${arr[@]}"; do ... # ✓
   ```

3. **Filling an array from `$(ls)`.**
   ```bash
   files=($(ls))    # ❌ file names with spaces break
   mapfile -t files < <(ls)   # ✓
   ```

4. **`unset` does not give the indices back.**
   After `unset 'arr[1]'`, `${#arr[@]}` is not 2, and `${arr[2]}` still exists.
   The fix: compact with `arr=("${arr[@]}")`.

5. **Associative arrays don't work on macOS.**
   The default bash 3.2 — `declare -A` throws an error. You need `brew install bash`.

6. **Mixing up negative slicing and the default value.**
   ```bash
   "${arr[@]:-2}"     # ❌ default value
   "${arr[@]: -2}"    # ✓ last 2 elements (the space!)
   ```

7. **Unquoted string joining.**
   ```bash
   IFS=',' echo "${arr[*]}"     # ❌ echo doesn't see the IFS
   (IFS=','; echo "${arr[*]}")  # ✓ inside a subshell
   ```
:::

---

## 2.10. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **6** exercises with auto-checking via the `bashlings` CLI:

```bash
bashlings watch              # start from the first pending exercise
bashlings run arr1           # check a single exercise
bashlings hint arr1          # step-by-step hint
```

Source: [`exercises/07_arrays/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/07_arrays)
:::

And try the following conceptual exercises by hand yourself:

1. **`reverse_array`** — write a function that prints an array in reverse order.
   Hint: `for ((i=${#arr[@]}-1; i>=0; i--))`

2. **`unique_items`** — return the elements of an array without duplicates. Hint: combine with `sort -u`.

3. **`csv_to_dict`** — a function that converts a string in the format `"key1=val1,key2=val2,..."` into an associative array.

4. **`top_n`** — takes a numeric array and an argument `n`, and returns the largest `n` numbers. The `sort -rn | head` pattern.

5. **`group_by_ext`** — group the files in the current directory by their extension (associative array). Example result:
   ```
   md: [README.md doc.md]
   sh: [build.sh test.sh]
   ```

---

## 2.11. Summary

| Concept                         | Key point                                          |
|---------------------------------|----------------------------------------------------|
| **Indexed array**               | `arr=(a b c)` or `declare -a`                      |
| **Associative array**           | `declare -A` (required, Bash 4+)                   |
| **All elements**                | `"${arr[@]}"` (each element separately)            |
| **A single string**             | `"${arr[*]}"` (joined with IFS)                    |
| **Element count**               | `${#arr[@]}`                                       |
| **Keys/indices**                | `"${!arr[@]}"`                                     |
| **Appending**                   | `arr+=("yangi")`                                   |
| **Slicing**                     | `"${arr[@]:start:count}"`                          |
| **File → array**                | `mapfile -t arr < fayl.txt`                        |
| **Splitting a string**          | `IFS=',' read -ra arr <<< "$s"`                    |
| **Macro quoting**               | Always `"${arr[@]}"` — unquoted is dangerous       |

### Key ideas

1. **An array = storing multiple values.** A string is a single value.
2. **Always use `@`** — use `*` only for joining into a string.
3. **macOS bash 3.2** — associative arrays don't work, you need brew bash.
4. **`mapfile`** — the safest way from a file into an array.
5. **`unset` leaves residue** — compact with `arr=("${arr[@]}")`.

🎉 Now you have the skill to use powerful data structures in Bash. In the next chapter we will learn industrial-grade text processing with **`sed`** and **`awk`**.

> **Next page:** [3. Mastering sed, awk and grep →](./03-sed-awk)
