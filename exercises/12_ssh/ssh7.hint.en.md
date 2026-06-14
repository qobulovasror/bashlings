# 💡 ssh7

## Step 1
Filter only the successful logins:
```bash
echo "$log" | grep "Accepted publickey"
```

## Step 2
Example of one line:
```
May 26 08:14:01 host sshd[1234]: Accepted publickey for deploy from 10.0.0.5 ...
 $1   $2   $3   $4     $5         $6     $7        $8   $9     $10  $11
```

The USER name is the 9th column:
```bash
awk '{print $9}'
```

## Step 3
Unique and in alphabetical order:
```bash
sort -u
```

`sort -u` = equivalent to `sort | uniq`.
