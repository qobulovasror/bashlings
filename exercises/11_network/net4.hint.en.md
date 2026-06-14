# 💡 net4

## Step 1
The command template:
```
curl -X POST -H 'Content-Type: application/json' -d '<JSON>' <URL>
```

## Step 2
In Bash a variable does **NOT expand** inside SINGLE quotes (`'`). However, single
quotes may appear inside `echo`'s double quotes (`"`) — there bash treats the
single quote as a literal character:
```bash
echo "curl -H 'X-Foo: bar'"
# curl -H 'X-Foo: bar'
```

## Step 3
`$BODY` contains `"`, but we `echo` it inside double quotes —
so the value of `BODY` becomes EXACTLY the body string (no escaping needed,
because BODY is itself a single-quoted string).

```bash
echo "curl -X POST -H 'Content-Type: application/json' -d '$BODY' $URL"
```
