---
title: "CI/CD — GitHub Actions"
description: "GitHub Actions workflows, ShellCheck, bats tests, matrix builds, secrets, Docker push, and release automation."
---

# 6. CI/CD — GitHub Actions

> **🎯 What you will learn in this chapter:**
> - The basics of **CI/CD** and the GitHub Actions model
> - The **Workflow → Job → Step** hierarchy
> - Triggers — `push`, `pull_request`, `schedule`, `workflow_dispatch`
> - **ShellCheck + bats-core** integration
> - **Matrix builds** — Linux + macOS, in parallel at the same time
> - Secrets, env vars, action pinning (security)
> - **Docker** and **release** automation
> - Real example — a **complete CI** pipeline for bashlings
>
> **⏱ Time:** ~35 minutes
> **🧪 Exercises:** `bashlings watch` — 7 interactive exercises ready ([`exercises/16_cicd/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/16_cicd))

---

## 6.1. What is CI/CD?

**Continuous Integration (CI)** — on every commit the code is automatically checked: build, test, lint.

**Continuous Delivery/Deployment (CD)** — code that passes the tests is automatically deployed.

```
Developer push   →   CI: build + test + lint   →   CD: deploy
       │              │                              │
   git commit      GitHub Actions               Server/registry
```

::: tip Core idea
The "it worked on my machine" problem — in CI, on **EVERY push** everything is built and checked in a clean environment. It is made **mandatory** that your script passes `shellcheck` and that the `bats` tests succeed.
:::

---

## 6.2. GitHub Actions — the most popular platform

GitHub Actions — GitHub's built-in CI/CD platform. **Free** for public repos, with a limited free quota for private repos.

### Structure

```
Repository
└── .github/
    └── workflows/
        ├── ci.yml          # on every push
        ├── release.yml     # on tag push
        └── nightly.yml     # on a schedule
```

Each `.yml` file is a single **workflow**. A workflow contains several **jobs**, and each job contains several **steps**.

### YAML basics

```yaml
name: CI                     # workflow name (shown in the GitHub UI)

on:                          # when it runs
  push:
    branches: [main]
  pull_request:

jobs:                        # several parallel jobs
  test:                      # job name
    runs-on: ubuntu-latest   # which VM
    steps:                   # steps executed in sequence
      - uses: actions/checkout@v4
      - run: ./tests.sh
```

---

## 6.3. Triggers (`on:`)

| Trigger              | Meaning                                       |
|----------------------|-----------------------------------------------|
| `push`               | A commit pushed to a branch                   |
| `pull_request`       | When a PR is created / updated                |
| `schedule`           | Cron syntax (hourly, daily, ...)              |
| `workflow_dispatch`  | Manual run from the UI                        |
| `release`            | When a new release is created                 |
| `workflow_call`      | Called from another workflow (reusable)       |

### Triggers in an example

```yaml
on:
  # Push to specific branches
  push:
    branches: [main, develop]
    paths-ignore:
      - '**.md'              # ignore markdown

  # PRs
  pull_request:
    branches: [main]

  # Cron — every day at 02:00 UTC
  schedule:
    - cron: '0 2 * * *'

  # Manual
  workflow_dispatch:
    inputs:
      environment:
        description: "Qaysi environment?"
        required: true
        type: choice
        options:
          - staging
          - prod
```

---

## 6.4. Job and Steps

### Job — runs on a separate virtual machine

```yaml
jobs:
  shellcheck:
    runs-on: ubuntu-latest         # official runners
    timeout-minutes: 5             # mandatory timeout
    steps:
      - uses: actions/checkout@v4
      - run: shellcheck *.sh

  bats-tests:
    runs-on: ubuntu-latest
    needs: shellcheck              # after shellcheck is OK
    steps:
      - uses: actions/checkout@v4
      - run: bats tests/
```

`needs:` — a dependency between jobs. **By default, all jobs run in parallel**.

### Step — two kinds

**1. `uses:`** — use another action:
```yaml
- uses: actions/checkout@v4
- uses: actions/setup-node@v4
  with:
    node-version: '20'
```

**2. `run:`** — a shell command:
```yaml
- name: Test
  run: |
    echo "Salom CI"
    npm ci
    npm test
```

`|` — multi-line YAML. The default shell is `bash` (Linux/macOS) or `pwsh` (Windows).

### Popular runners

| Runner              | Description                          |
|---------------------|--------------------------------------|
| `ubuntu-latest`     | Ubuntu LTS (24.04 currently)         |
| `ubuntu-22.04`      | Exact version                        |
| `macos-latest`      | macOS (ARM64, expensive — 10×)       |
| `macos-14`          | Intel macOS                          |
| `windows-latest`    | Windows Server                       |
| `self-hosted`       | Your own server                      |

::: warning macOS runner — expensive
The macOS runner is **10× more expensive** than Linux (on a private repo). Use it only when necessary. For cross-platform testing, optimize with a matrix (§6.7).
:::

---

## 6.5. ShellCheck integration

In every Bash repo, **shellcheck** is mandatory in CI.

### Simple variant

```yaml
jobs:
  shellcheck:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: ShellCheck
        run: |
          sudo apt-get install -y shellcheck
          find . -name '*.sh' -print0 | xargs -0 shellcheck
```

### With the `koalaman/shellcheck` action

```yaml
jobs:
  shellcheck:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ludeeus/action-shellcheck@master
        with:
          scandir: 'exercises'
          severity: warning
```

Configuration via `.shellcheckrc`:
```
# .shellcheckrc
disable=SC2086     # word splitting ignored — global
```

---

## 6.6. Testing with `bats-core`

[bats-core](https://github.com/bats-core/bats-core) — the official testing framework for Bash.

### Example test file

```bash
# tests/test_math.bats

@test "yigindi 2 va 3" {
    result=$(./math.sh 2 3)
    [ "$result" = "5" ]
}

@test "argumentsiz xato beradi" {
    run ./math.sh
    [ "$status" -ne 0 ]
}

@test "katta sonlar ham ishlaydi" {
    result=$(./math.sh 1000 2000)
    [ "$result" = "3000" ]
}
```

### In CI

```yaml
jobs:
  bats:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install bats
        run: sudo apt-get install -y bats
      - name: Run tests
        run: bats tests/
```

---

## 6.7. Matrix builds — Linux + macOS at the same time

```yaml
jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false           # if one job fails, let the others continue
      matrix:
        os: [ubuntu-latest, macos-latest]
        bash: [4, 5]
    steps:
      - uses: actions/checkout@v4
      - name: Install bash ${{ matrix.bash }}
        run: |
          if [[ "$RUNNER_OS" == "macOS" && "${{ matrix.bash }}" == "5" ]]; then
            brew install bash
          fi
      - run: bash --version
      - run: bats tests/
```

This matrix produces **4** jobs (2 OS × 2 bash). They run in parallel.

### Conditional include/exclude

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest]
    bash: [4, 5]
    exclude:
      - os: macos-latest
        bash: 4              # there is no bash 4 on macOS
    include:
      - os: ubuntu-latest
        bash: 3.2            # bonus — test old bash
```

---

## 6.8. Secrets and environment variables

### Repo Secrets

Create them via `Settings → Secrets and variables → Actions`.

```yaml
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Deploy
        env:
          API_KEY: ${{ secrets.API_KEY }}
          SLACK_WEBHOOK: ${{ secrets.SLACK_WEBHOOK }}
        run: |
          curl -fsS -H "Authorization: Bearer $API_KEY" https://api.example.com/deploy
```

### `GITHUB_TOKEN` — built-in

Automatically available in every workflow — for writing to and reading from the repo:

```yaml
- name: Create release
  env:
    GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  run: gh release create v1.0.0 ./binary
```

### Hiding secrets in the log

GitHub Actions **automatically** replaces secrets in the log with `***`. But:

::: danger Do not print secrets to the log
```yaml
run: echo $API_KEY        # ❌ DANGEROUS — appears in the log
run: echo "deploying..."  # ✓
```

Even if you `echo` it, GitHub replaces it with `***`, but it may leak through the **error log** or in **debug** mode.
:::

### `vars:` (non-secret values)

For configuration that may be public in the repo:

```yaml
env:
  REGION: ${{ vars.AWS_REGION }}
  ENV: ${{ vars.DEPLOY_ENV }}
```

---

## 6.9. Cache and artifacts

### `actions/cache` — dependency caching

```yaml
- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo
      target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    restore-keys: |
      ${{ runner.os }}-cargo-
```

Reduces the Rust build time by **10×**. For Node it's `~/.npm`, for Python `~/.cache/pip`, and so on.

### Artifacts — build results

```yaml
- name: Build
  run: cargo build --release

- uses: actions/upload-artifact@v4
  with:
    name: bashlings-${{ runner.os }}
    path: target/release/bashlings

# Another job
- uses: actions/download-artifact@v4
  with:
    name: bashlings-ubuntu-latest
```

Artifacts are kept for 90 days (default). They can be downloaded through the UI.

---

## 6.10. Docker in CI

### Docker build + push

```yaml
jobs:
  docker:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write           # to write to ghcr.io
    steps:
      - uses: actions/checkout@v4
      
      - uses: docker/setup-buildx-action@v3
      
      - uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}
      
      - uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: |
            ghcr.io/${{ github.repository }}:latest
            ghcr.io/${{ github.repository }}:${{ github.sha }}
          cache-from: type=gha
          cache-to: type=gha,mode=max
```

### Services — for DB tests

```yaml
jobs:
  integration:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_PASSWORD: test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    steps:
      - uses: actions/checkout@v4
      - run: |
          export PGPASSWORD=test
          psql -h localhost -U postgres -c 'SELECT 1'
          ./run-integration.sh
```

It starts the Docker container **on the CI VM** and does port forwarding.

---

## 6.11. Release automation

### Tag-based release

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'                # v1.0.0, v0.2.1, ...

jobs:
  release:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4
      
      - name: Build
        run: cargo build --release
      
      - name: Create release
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          gh release create "${{ github.ref_name }}" \
            --title "Release ${{ github.ref_name }}" \
            --generate-notes \
            ./target/release/bashlings
```

### Cross-platform binaries

```yaml
strategy:
  matrix:
    include:
      - os: ubuntu-latest
        target: x86_64-unknown-linux-gnu
      - os: macos-latest
        target: aarch64-apple-darwin
      - os: macos-latest
        target: x86_64-apple-darwin

steps:
  - uses: dtolnay/rust-toolchain@stable
    with:
      targets: ${{ matrix.target }}
  
  - run: cargo build --release --target ${{ matrix.target }}
  
  - name: Upload to release
    env:
      GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    run: |
      cp target/${{ matrix.target }}/release/bashlings bashlings-${{ matrix.target }}
      gh release upload "${{ github.ref_name }}" bashlings-${{ matrix.target }}
```

The user downloads the binary for their OS from the `Releases` page.

---

## 6.12. Conditional execution — `if:`

```yaml
steps:
  - name: Notify Slack on failure
    if: failure()
    run: ./notify.sh "CI failed"

  - name: Deploy only on main
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    run: ./deploy.sh

  - name: macOS-specific step
    if: runner.os == 'macOS'
    run: brew install some-tool

  - name: Always run cleanup
    if: always()
    run: ./cleanup.sh
```

Status helpers: `success()`, `failure()`, `cancelled()`, `always()`.

---

## 6.13. Reusable workflows

Shared CI logic across several repos:

**`.github/workflows/reusable-lint.yml`:**
```yaml
on:
  workflow_call:
    inputs:
      scandir:
        type: string
        default: '.'

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ludeeus/action-shellcheck@master
        with:
          scandir: ${{ inputs.scandir }}
```

Calling it from another workflow:
```yaml
jobs:
  shellcheck:
    uses: your-org/shared-actions/.github/workflows/reusable-lint.yml@v1
    with:
      scandir: 'exercises'
```

---

## 6.14. Real example — a complete CI for `bashlings`

```yaml
# .github/workflows/ci.yml

name: CI

on:
  push:
    branches: [main]
  pull_request:

env:
  RUST_BACKTRACE: 1

jobs:
  # === 1. Lint the shell scripts ===
  shellcheck:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run ShellCheck
        uses: ludeeus/action-shellcheck@master
        with:
          scandir: '.'
          ignore_paths: cli/target
          severity: warning

  # === 2. CLI build and test (matrix) ===
  cli:
    name: CLI / ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@stable
      
      - uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            cli/target
          key: ${{ runner.os }}-cargo-${{ hashFiles('cli/Cargo.lock') }}
      
      - name: Build
        working-directory: cli
        run: cargo build --release
      
      - name: Test --help
        run: ./cli/target/release/bashlings --help
      
      - name: Test --version
        run: ./cli/target/release/bashlings --version

  # === 3. Confirm that all solutions work correctly ===
  validate-solutions:
    name: Solutions validation
    runs-on: ubuntu-latest
    needs: cli
    steps:
      - uses: actions/checkout@v4
      
      - name: Install bash 5
        run: sudo apt-get install -y bash
      
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Build bashlings
        run: cd cli && cargo build --release
      
      - name: Validate each solution
        run: |
          set -euo pipefail
          fail=0
          for sol in solutions/*/*.sh; do
              dir=$(basename "$(dirname "$sol")")
              name=$(basename "$sol" .sh)
              ex="exercises/$dir/$name.sh"
              cp "$ex" "/tmp/$name.bak"
              cp "$sol" "$ex"
              if ! ./cli/target/release/bashlings run "$name" > /dev/null 2>&1; then
                  echo "❌ $name yiqildi"
                  fail=$((fail+1))
              else
                  echo "✓ $name"
              fi
              mv "/tmp/$name.bak" "$ex"
          done
          [[ $fail -eq 0 ]] || exit 1

  # === 4. VitePress book build ===
  book:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      - run: npm ci
      - run: npm run docs:build
      - uses: actions/upload-artifact@v4
        with:
          name: docs-dist
          path: docs/.vitepress/dist
          retention-days: 7
```

### What does this CI do?

| Job                  | What it checks                                      |
|----------------------|-----------------------------------------------------|
| `shellcheck`         | All `.sh` files pass lint                            |
| `cli` (matrix)       | The CLI builds on Linux + macOS                     |
| `validate-solutions` | Confirms the correctness of **60 solutions**        |
| `book`               | The VitePress book builds without errors            |

When you push — 4 jobs run in parallel. If they are all green, the PR can be merged.

### Release workflow

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  build-and-release:
    name: ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    permissions:
      contents: write
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: macos-13
            target: x86_64-apple-darwin
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Build
        working-directory: cli
        run: cargo build --release --target ${{ matrix.target }}
      
      - name: Package
        run: |
          mkdir release
          cp cli/target/${{ matrix.target }}/release/bashlings \
             release/bashlings-${{ matrix.target }}
      
      - name: Upload to GitHub Release
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          gh release create "${{ github.ref_name }}" --generate-notes 2>/dev/null || true
          gh release upload "${{ github.ref_name }}" \
              release/bashlings-${{ matrix.target }}
```

Now `git tag v0.2.0 && git push --tags` — an automatic binary release for 3 platforms.

---

## 6.15. Other CI platforms (briefly)

| Platform        | Configuration file        | Note                                 |
|-----------------|---------------------------|--------------------------------------|
| **GitHub Actions** | `.github/workflows/*.yml` | The most popular                   |
| **GitLab CI**   | `.gitlab-ci.yml`          | For GitLab, very powerful           |
| **CircleCI**    | `.circleci/config.yml`    | Commercial                          |
| **Drone**       | `.drone.yml`              | Self-hosted, simple                 |
| **Jenkins**     | `Jenkinsfile`             | Old-and-new, lots of plugins        |
| **Buildkite**   | `.buildkite/pipeline.yml` | Hybrid (cloud + self-hosted runners)|

The syntax differs, but **the logic is the same**: trigger → job → step → script.

---

## 6.16. Common mistakes

::: danger Classic pitfalls

1. **`echo`-ing secrets to the log.**
   GitHub automatically masks them with `***`, but they can leak through **debug mode** or **error messages**.

2. **`actions/...@main` — pinning to a SHA is recommended.**
   ```yaml
   - uses: actions/checkout@v4                              # OK
   - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11  # PARANOID (safer)
   ```

3. **macOS runner overuse.**
   macOS is expensive (10×). Only when necessary (cross-platform binary, macOS-specific test).

4. **The default `bash` has NO strict mode.**
   In GitHub Actions a `run: |` block defaults to `bash --noprofile --norc -eo pipefail`. But `-u` is missing. Add it yourself.

5. **The working directory resets on every step.**
   ```yaml
   - run: cd dist          # this lasts only until this step
   - run: ls               # now back at the root again!
   ```
   Solution: `working-directory: dist` on the step.

6. **Writing `if:` conditions incorrectly.**
   ```yaml
   if: github.ref == 'refs/heads/main'        # ✓
   if: ${{ github.ref == 'refs/heads/main' }} # OK but unnecessary
   if: github.ref = 'main'                    # ❌ error
   ```

7. **A simple cache key — rebuilds every time.**
   ```yaml
   key: cargo                          # ❌ static — never updates
   key: cargo-${{ hashFiles('**/Cargo.lock') }}   # ✓ updates when lock changes
   ```

8. **`permissions:` not set — fail.**
   By default, `GITHUB_TOKEN` only has read access. If you need to write (release, push image):
   ```yaml
   permissions:
     contents: write
   ```

9. **Not being able to test a workflow file on a branch.**
   A workflow may not run on a PR (security). **Be careful** about using `pull_request_target` (code injection risk).

10. **`schedule` triggers — UTC.**
    Cron syntax is in UTC time. A local 02:00 ≠ `0 2 * * *`.
:::

---

## 6.17. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **7** exercises, with auto-checking via the `bashlings` CLI. None of them
require a GitHub Actions runner — they work with YAML and shell parsing:

```bash
bashlings watch              # start from the first pending exercise
bashlings run cicd1          # check a single exercise
bashlings hint cicd1         # step-by-step hint
```

Source: [`exercises/16_cicd/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/16_cicd)
:::

Try the following real-world exercises somewhere you have a repo on GitHub:

1. **First workflow** — create `.github/workflows/ci.yml` in a repo. On `push`, have it print `echo "Hello CI"`.

2. **ShellCheck CI** — add a ShellCheck action for the current project (or any repo). Try making an error appear in a PR.

3. **Matrix** — write a job that shows bash versions on Ubuntu + macOS (`bash --version` on each OS).

4. **Deploy with a secret** — using a fake secret (`secrets.MY_TOKEN`), have it print "**`Token uzunligi: N`**". Verify that GitHub keeps it hidden.

5. **Release pipeline** — a minimal workflow that, on a tag push, builds a binary and uploads it to GitHub Releases.

---

## 6.18. Summary

| Concept                     | Key point                                          |
|-----------------------------|----------------------------------------------------|
| Workflow                    | `.github/workflows/*.yml`                          |
| Trigger `on:`               | `push`, `pull_request`, `schedule`, `workflow_dispatch` |
| Job                         | A parallel unit running on a separate VM           |
| `runs-on:`                  | `ubuntu-latest`, `macos-latest`, ...               |
| `needs:`                    | Job dependency                                     |
| `steps:`                    | `uses:` (action) or `run:` (shell)                 |
| <span v-pre>`${{ secrets.X }}`</span> | Read from repo Secrets                             |
| <span v-pre>`${{ matrix.os }}`</span> | Access a matrix value                              |
| `actions/checkout@v4`       | The classic first step                             |
| `actions/cache@v4`          | Reduce build time by 10×                           |
| `actions/upload-artifact@v4`| Save build results                                 |
| `if: failure()`             | Only when the previous step failed                 |
| `permissions:`              | Scope of `GITHUB_TOKEN`                            |
| `strategy.matrix`           | Multiple variants at the same time                 |
| Reusable workflow           | `workflow_call` + `uses:` to another workflow      |

### 5 core ideas

1. **`shellcheck` CI in every Bash repo** — non-negotiable.
2. **Matrix tests** — Linux + macOS at minimum.
3. **`actions/cache@v4`** — on every dependency-heavy build.
4. **Never print secrets to the log.**
5. **Pin your actions** — `@v4` is enough too, but the paranoid pin to a SHA.

---

## 🎉 The book is finished!

You have traveled the path from **beginner to fully professional** in the Bash & Linux ecosystem:

| Part    | Chapters | Topics                                              |
|---------|--------|-------------------------------------------------------|
| **Part 1** | 5    | Terminal, navigation, pipes, text processing, your first script |
| **Part 2** | 5    | Functions, arrays, sed/awk, traps, robust scripting |
| **Part 3** | 6    | Networking, SSH, jq, cron, Docker, CI/CD             |

**Total:** 16 chapters, ~9,000 lines of markdown, 60+ interactive exercises, the **`bashlings`** CLI.

### What can you do now?

- ✅ **Server administration** — work daily with SSH
- ✅ **DevOps automation** — cron, systemd, CI/CD pipelines
- ✅ **API integration** — curl + jq at production level
- ✅ **Containerization** — sandbox and deploy with Docker
- ✅ **Production scripts** — `set -euo pipefail`, trap, ShellCheck
- ✅ **Team skills** — `~/.ssh/config`, alias, dotfiles

### Next steps

1. **Create your own dotfiles** — `~/.bashrc`, `~/.ssh/config`, `~/.tmux.conf` on GitHub
2. **Get involved in a real project** — send PRs to open source repos
3. **Connect CI/CD to a real deploy** — build a complete pipeline for one of your projects
4. **Move into a teaching role** — teach someone else, the best way to learn

### Remember

> *"The best way to learn Bash is to try to solve real problems with Bash every day."*

Now the terminal is yours. Use it. Automate. Build.

🚀 **Good luck!**

---

📘 **[← Back to the home page](/en/)**
