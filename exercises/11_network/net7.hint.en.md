# 💡 net7

## Step 1
The classic template for looping over an array:
```bash
for x in "${ARR[@]}"; do
    echo "$x"
done
```

Here `"${ARR[@]}"` passes each element as a separate argument
(no word-splitting problem).

## Step 2
Capturing a function's output — command substitution:
```bash
result=$(curl "$url")
echo "$result"   # for example: 200
```

## Step 3
Format `URL=STATUS`:
```bash
status=$(curl "$url")
echo "$url=$status"
```

Place these inside the for loop.
