# 💡 script1 — Hints

## Step 1
When declaring a variable there must be **no spaces** around `=`:
```bash
name="Bash"      # ✅ correct
name = "Bash"    # ❌ WRONG — bash treats this as a command
```

## Step 2
Inside "..." double quotes `$variable` is substituted automatically:
```bash
echo "Salom, $name!"   # Salom, Bash!
```

Inside '...' single quotes nothing is substituted:
```bash
echo 'Salom, $name!'   # Salom, $name! — literal
```
