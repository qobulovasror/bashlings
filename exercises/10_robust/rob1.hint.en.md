# 💡 rob1

## Step 1
Default Bash behavior — we take the exit code of the **last** command in the pipeline:
```bash
false | true   # exit 0 (true's exit)
```

This is dangerous — errors inside the pipeline are hidden.

## Step 2
`set -o pipefail` — now the rightmost non-zero exit code of the pipeline is returned:
```bash
set -o pipefail
false | true   # exit 1 (false's exit)
```
