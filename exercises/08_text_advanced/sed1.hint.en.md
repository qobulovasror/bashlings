# 💡 sed1

## Step 1
`sed 's/A/B/'` — replaces the first `A` with `B` on each line.

## Step 2
On each line `old` appears only once, so `g` isn't needed. But it's safe to always use `g` anyway.
