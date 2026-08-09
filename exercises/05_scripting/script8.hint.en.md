# 💡 script8 — Hints

## Step 1
`read` takes **one line** from stdin and stores it in a variable:

```bash
read -r name
echo "Salom, $name!"
```

`-r` treats a backslash as an ordinary character. You almost always want it —
without `-r`, text like `C:\temp` gets mangled.

## Step 2
`read` consumes only that one line — the remaining lines stay on stdin, so the
stream is not finished after the first `read`.

## Step 3
Count the rest with a `while read` loop, which spins until the stream ends:

```bash
qolgan=0
while read -r _; do
    qolgan=$((qolgan + 1))
done
echo "Qolganlari: $qolgan"
```

`_` is the conventional name for "I don't need this value".
