# 💡 arr5

## Step 1
**Associative arrays** require Bash 4+. macOS stock bash 3.2 won't work:
```bash
bash --version   # must be 4.x or 5.x
brew install bash  # if it is 3.x
```

## Step 2
Declaring and filling:
```bash
declare -A user
user[name]="Ali"
user[city]="Toshkent"
```

Or all at once:
```bash
declare -A user=([name]="Ali" [city]="Toshkent")
```
