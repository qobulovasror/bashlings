# 💡 script4 — Hints

## Step 1
The 3 main forms of a `for` loop in Bash:
```bash
# A. Over a list
for x in 1 2 3 4 5; do echo "$x"; done

# B. Range (brace expansion)
for x in {1..5}; do echo "$x"; done

# C. C-style
for ((i=1; i<=5; i++)); do echo "$i"; done
```

## Step 2
In this exercise the shortest is **variant B**.

## Step 3
The code to run goes inside the `do ... done` block.
