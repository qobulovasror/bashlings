---
title: "CI/CD — GitHub Actions"
description: "GitHub Actions workflow'lari, ShellCheck, bats testlari, matrix builds, secrets, Docker push va release avtomatlashtirish."
---

# 6. CI/CD — GitHub Actions

> **🎯 Bu bobda nimani o'rganasiz:**
> - **CI/CD** asoslari va GitHub Actions modeli
> - **Workflow → Job → Step** ierarxiyasi
> - Triggerlar — `push`, `pull_request`, `schedule`, `workflow_dispatch`
> - **ShellCheck + bats-core** integratsiyasi
> - **Matrix builds** — Linux + macOS, bir vaqtda parallel
> - Secrets, env vars, action pinning (security)
> - **Docker** va **release** avtomatlashtirish
> - Real misol — **bashlings uchun to'liq CI** pipeline
>
> **⏱ Vaqt:** ~35 daqiqa
> **🧪 Mashqlar:** `bashlings watch` — 7 ta interaktiv mashq tayyor ([`exercises/16_cicd/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/16_cicd))

---

## 6.1. CI/CD nima?

**Continuous Integration (CI)** — har commit'da kod avtomatik tekshiriladi: build, test, lint.

**Continuous Delivery/Deployment (CD)** — testlardan o'tgan kod avtomatik deploy qilinadi.

```
Developer push   →   CI: build + test + lint   →   CD: deploy
       │              │                              │
   git commit      GitHub Actions               Server/registry
```

::: tip Asosiy g'oya
"Mening kompyuterimda ishladi" muammosi — CI'da **HAR push** da hamma narsa toza muhitda quriladi va tekshiriladi. Skriptingiz `shellcheck`'dan o'tishi va `bats` testlari muvaffaqiyatli bo'lishi **majburiy** qilinadi.
:::

---

## 6.2. GitHub Actions — eng mashhur platform

GitHub Actions — GitHub'ning built-in CI/CD platformasi. Public repolar uchun **bepul**, private repolar uchun cheklangan tekin kvota.

### Tuzilishi

```
Repository
└── .github/
    └── workflows/
        ├── ci.yml          # har push'da
        ├── release.yml     # tag push'da
        └── nightly.yml     # schedule bo'yicha
```

Har `.yml` fayl — bitta **workflow**. Workflow ichida bir nechta **job**, har job ichida — bir nechta **step**.

### YAML asoslari

```yaml
name: CI                     # workflow nomi (GitHub UI'da ko'rinadi)

on:                          # qachon ishga tushadi
  push:
    branches: [main]
  pull_request:

jobs:                        # bir nechta parallel job
  test:                      # job nomi
    runs-on: ubuntu-latest   # qaysi VM'da
    steps:                   # ketma-ket bajariladigan qadamlar
      - uses: actions/checkout@v4
      - run: ./tests.sh
```

---

## 6.3. Triggerlar (`on:`)

| Trigger              | Mazmuni                                       |
|----------------------|-----------------------------------------------|
| `push`               | Branch'ga commit push'i                       |
| `pull_request`       | PR yaratilganida / yangilanganida             |
| `schedule`           | Cron syntax (har soat, kun, ...)              |
| `workflow_dispatch`  | UI'dan qo'lda ishga tushirish                 |
| `release`            | Yangi release yaratilganida                    |
| `workflow_call`      | Boshqa workflow'dan chaqirish (reusable)      |

### Triggers misolda

```yaml
on:
  # Push to specific branches
  push:
    branches: [main, develop]
    paths-ignore:
      - '**.md'              # markdown'ni e'tibordan tashqarida

  # PR'lar
  pull_request:
    branches: [main]

  # Cron — har kun 02:00 UTC
  schedule:
    - cron: '0 2 * * *'

  # Qo'lda
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

## 6.4. Job va Steps

### Job — alohida virtual mashinada ishlaydi

```yaml
jobs:
  shellcheck:
    runs-on: ubuntu-latest         # rasmiy runners
    timeout-minutes: 5             # majburiy timeout
    steps:
      - uses: actions/checkout@v4
      - run: shellcheck *.sh

  bats-tests:
    runs-on: ubuntu-latest
    needs: shellcheck              # shellcheck OK bo'lgandan keyin
    steps:
      - uses: actions/checkout@v4
      - run: bats tests/
```

`needs:` — job'lar orasidagi dependency. **Default'da hamma job'lar parallel**.

### Step — ikki turi

**1. `uses:`** — boshqa action'dan foydalanish:
```yaml
- uses: actions/checkout@v4
- uses: actions/setup-node@v4
  with:
    node-version: '20'
```

**2. `run:`** — shell buyrug'i:
```yaml
- name: Test
  run: |
    echo "Salom CI"
    npm ci
    npm test
```

`|` — multi-line YAML. Default shell — `bash` (Linux/macOS) yoki `pwsh` (Windows).

### Mashhur runner'lar

| Runner              | Ta'rifi                              |
|---------------------|--------------------------------------|
| `ubuntu-latest`     | Ubuntu LTS (24.04 hozirda)           |
| `ubuntu-22.04`      | Aniq versiya                          |
| `macos-latest`      | macOS (ARM64, qimmat — 10×)          |
| `macos-14`          | Intel macOS                          |
| `windows-latest`    | Windows Server                       |
| `self-hosted`       | O'zingizning serveringiz             |

::: warning macOS runner — qimmat
macOS runner Linux'dan **10× qimmat** (private repo'da). Faqat zarurat bo'lsa ishlating. Cross-platform test uchun matrix bilan optimization (§6.7).
:::

---

## 6.5. ShellCheck integratsiyasi

Har Bash repo CI'da **shellcheck** majburiy.

### Sodda variant

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

### `koalaman/shellcheck` action bilan

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

`.shellcheckrc` orqali konfiguratsiya:
```
# .shellcheckrc
disable=SC2086     # word splitting tashqarida — global
```

---

## 6.6. `bats-core` bilan testing

[bats-core](https://github.com/bats-core/bats-core) — Bash uchun rasmiy testing framework.

### Test fayl misoli

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

### CI'da

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

## 6.7. Matrix builds — Linux + macOS bir vaqtda

```yaml
jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false           # bir job yiqilsa, boshqalari davom etsin
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

Bu matrix — **4 ta** job hosil qiladi (2 OS × 2 bash). Parallel bajariladi.

### Conditional include/exclude

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest]
    bash: [4, 5]
    exclude:
      - os: macos-latest
        bash: 4              # macOS'da bash 4 yo'q
    include:
      - os: ubuntu-latest
        bash: 3.2            # bonus — eski bash test
```

---

## 6.8. Secrets va environment variables

### Repo Secrets

`Settings → Secrets and variables → Actions` orqali yarating.

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

Har workflow'da avtomatik mavjud — repo'ga yozish va read uchun:

```yaml
- name: Create release
  env:
    GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  run: gh release create v1.0.0 ./binary
```

### Secrets'ni log'da yashirish

GitHub Actions **avtomatik** secrets'ni log'da `***` bilan almashtiradi. Lekin:

::: danger Secrets'ni log'ga chiqarmang
```yaml
run: echo $API_KEY        # ❌ XAVFLI — log'da ko'rinadi
run: echo "deploying..."  # ✓
```

Hatto `echo` qilsangiz ham — GitHub `***` bilan almashtiradi, lekin **error log'da** yoki **debug**'da sirib chiqishi mumkin.
:::

### `vars:` (sirsiz qiymatlar)

Repo'da public bo'lishi mumkin bo'lgan konfiguratsiya uchun:

```yaml
env:
  REGION: ${{ vars.AWS_REGION }}
  ENV: ${{ vars.DEPLOY_ENV }}
```

---

## 6.9. Cache va artifact'lar

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

Rust build vaqtini **10× kamaytiradi**. Node'da `~/.npm`, Python'da `~/.cache/pip` va h.k.

### Artifact'lar — build natijalari

```yaml
- name: Build
  run: cargo build --release

- uses: actions/upload-artifact@v4
  with:
    name: bashlings-${{ runner.os }}
    path: target/release/bashlings

# Boshqa job
- uses: actions/download-artifact@v4
  with:
    name: bashlings-ubuntu-latest
```

Artifact'lar 90 kun saqlanadi (default). UI orqali yuklab olish mumkin.

---

## 6.10. Docker'ni CI'da

### Docker build + push

```yaml
jobs:
  docker:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write           # ghcr.io ga yozish uchun
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

### Services — DB tests uchun

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

Docker konteynerni **CI VM'da** ishga tushiradi va port forwarding qiladi.

---

## 6.11. Release avtomatlashtirish

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

### Cross-platform binary'lar

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

Foydalanuvchi `Releases` sahifasidan o'z OS uchun binary'ni yuklab oladi.

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

Status helper'lar: `success()`, `failure()`, `cancelled()`, `always()`.

---

## 6.13. Reusable workflows

Bir nechta repo orasida umumiy CI logikasi:

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

Boshqa workflow'dan chaqirish:
```yaml
jobs:
  shellcheck:
    uses: your-org/shared-actions/.github/workflows/reusable-lint.yml@v1
    with:
      scandir: 'exercises'
```

---

## 6.14. Real misol — `bashlings` uchun to'liq CI

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
  # === 1. Shell skriptlarni lint qilish ===
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

  # === 2. CLI build va test (matrix) ===
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

  # === 3. Hamma solutions to'g'ri ishlashini tasdiqlash ===
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

### Bu CI nima qiladi?

| Job                  | Tekshiradi                                          |
|----------------------|-----------------------------------------------------|
| `shellcheck`         | Hamma `.sh` fayllar lint'dan o'tadi                  |
| `cli` (matrix)       | CLI Linux + macOS'da build bo'ladi                  |
| `validate-solutions` | **60 ta solution** to'g'riligini tasdiqlaydi        |
| `book`               | VitePress kitob xato'siz build bo'ladi              |

Push qilganda — 4 ta job parallel ishga tushadi. Hammasi yashil bo'lsa — PR merge bo'lishi mumkin.

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

Endi `git tag v0.2.0 && git push --tags` — avtomatik 3 platforma uchun binary release.

---

## 6.15. Boshqa CI platformalar (qisqacha)

| Platform        | Konfiguratsiya fayl       | Eslatma                              |
|-----------------|---------------------------|--------------------------------------|
| **GitHub Actions** | `.github/workflows/*.yml` | Eng mashhuri                       |
| **GitLab CI**   | `.gitlab-ci.yml`          | GitLab uchun, juda kuchli           |
| **CircleCI**    | `.circleci/config.yml`    | Komersial                           |
| **Drone**       | `.drone.yml`              | Self-hosted, oddiy                  |
| **Jenkins**     | `Jenkinsfile`             | Eski-yangi, plugin'lar ko'p         |
| **Buildkite**   | `.buildkite/pipeline.yml` | Hybrid (cloud + self-hosted runners)|

Sintaksis farqli, **mantiq bir xil**: trigger → job → step → script.

---

## 6.16. Tez-tez uchraydigan xatolar

::: danger Klassik tuzoqlar

1. **Secrets'ni log'ga `echo` qilish.**
   GitHub avtomatik `***` qiladi, lekin **debug mode** yoki **error message**'lar orqali sirib chiqishi mumkin.

2. **`actions/...@main` — pin qilingan SHA tavsiya etiladi.**
   ```yaml
   - uses: actions/checkout@v4                              # OK
   - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11  # PARANOID (xavfsizroq)
   ```

3. **macOS runner overuse.**
   macOS — qimmat (10×). Faqat zarurat bo'lsa (cross-platform binary, macOS-specific test).

4. **Default `bash` strict mode YO'Q.**
   GitHub Actions `run: |` blok default `bash --noprofile --norc -eo pipefail`. Lekin `-u` yo'q. O'zingiz qo'shing.

5. **Working directory har step'da reset.**
   ```yaml
   - run: cd dist          # bu step'gacha qoladi
   - run: ls               # endi yana root'da!
   ```
   Yechim: `working-directory: dist` step'da.

6. **`if:` shartlarni xato yozish.**
   ```yaml
   if: github.ref == 'refs/heads/main'        # ✓
   if: ${{ github.ref == 'refs/heads/main' }} # OK lekin keraksiz
   if: github.ref = 'main'                    # ❌ xato
   ```

7. **Cache key oddiy — har gal qayta build.**
   ```yaml
   key: cargo                          # ❌ static — hech qachon yangilanmaydi
   key: cargo-${{ hashFiles('**/Cargo.lock') }}   # ✓ lock o'zgarsa yangilanadi
   ```

8. **`permissions:` o'rnatilmagan — fail.**
   Default'da `GITHUB_TOKEN` faqat o'qishga ruxsat. Yozish kerak bo'lsa (release, push image):
   ```yaml
   permissions:
     contents: write
   ```

9. **Workflow file'ni branch'da test qilolmaslik.**
   PR'da workflow ishlamasligi mumkin (security). `pull_request_target` ishlatishdan **ehtiyot bo'ling** (kod injection xavfi).

10. **`schedule` triggers — UTC.**
    Cron syntax UTC vaqti bilan. Mahalliy 02:00 ≠ `0 2 * * *`.
:::

---

## 6.17. Mashqlar

::: tip 🧪 Bashlings — interaktiv mashqlar
Bu bobning **7 ta** mashqi `bashlings` CLI orqali avto-tekshiruv bilan. Hammasi
GitHub Actions runner talab qilmaydi — YAML va shell parsing bilan ishlash:

```bash
bashlings watch              # birinchi pending mashqdan boshlash
bashlings run cicd1          # bitta mashqni tekshirish
bashlings hint cicd1         # bosqichli maslahat
```

Manba: [`exercises/16_cicd/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/16_cicd)
:::

Quyidagi real-world mashqlarni GitHub'da repo bor joyda sinab ko'ring:

1. **Birinchi workflow** — repo'da `.github/workflows/ci.yml` yarating. `push` da `echo "Hello CI"` chiqarsin.

2. **ShellCheck CI** — joriy loyiha (yoki istalgan repo) uchun ShellCheck action qo'shing. PR'da xato chiqishini sinab ko'ring.

3. **Matrix** — Ubuntu + macOS'da bash versiyalarini ko'rsatuvchi job yozing (`bash --version` har OS'da).

4. **Secret bilan deploy** — fake secret (`secrets.MY_TOKEN`) ishlatib, "**`Token uzunligi: N`**" deb chiqarsin. GitHub buni yashirinligini tekshiring.

5. **Release pipeline** — tag push'da binary build qilib, GitHub Releases'ga yuklovchi minimal workflow.

---

## 6.18. Xulosa

| Tushuncha                   | Asosiy nuqta                                       |
|-----------------------------|----------------------------------------------------|
| Workflow                    | `.github/workflows/*.yml`                          |
| Trigger `on:`               | `push`, `pull_request`, `schedule`, `workflow_dispatch` |
| Job                         | Alohida VM'da ishlovchi parallel unit              |
| `runs-on:`                  | `ubuntu-latest`, `macos-latest`, ...               |
| `needs:`                    | Job dependency                                     |
| `steps:`                    | `uses:` (action) yoki `run:` (shell)               |
| <span v-pre>`${{ secrets.X }}`</span> | Repo Secrets'dan o'qish                            |
| <span v-pre>`${{ matrix.os }}`</span> | Matrix qiymatiga kirish                            |
| `actions/checkout@v4`       | Klassik birinchi step                              |
| `actions/cache@v4`          | Build vaqtini 10× kamaytirish                      |
| `actions/upload-artifact@v4`| Build natijalarni saqlash                          |
| `if: failure()`             | Faqat oldingi step yiqilsa                         |
| `permissions:`              | `GITHUB_TOKEN` doirasi                             |
| `strategy.matrix`           | Bir vaqtda bir nechta variant                      |
| Reusable workflow           | `workflow_call` + `uses:` boshqa workflow'ga       |

### 5 ta asosiy g'oya

1. **Har Bash repo `shellcheck` CI** — non-negotiable.
2. **Matrix test'lar** — Linux + macOS minimal.
3. **`actions/cache@v4`** — har dependency-heavy build'da.
4. **Secrets'ni hech qachon log'da chiqarmang.**
5. **Action'larni pin qiling** — `@v4` ham yetarli, lekin paranoidlar SHA pin qiladi.

---

## 🎉 Kitob tugadi!

Siz Bash & Linux ekotizimida **boshlovchidan to'liq professional**gacha yo'lni bosib o'tdingiz:

| Qism    | Boblar | Mavzular                                              |
|---------|--------|-------------------------------------------------------|
| **1-qism** | 5    | Terminal, navigatsiya, pipes, matn ishlash, birinchi skript |
| **2-qism** | 5    | Funksiyalar, massivlar, sed/awk, traps, robust scripting |
| **3-qism** | 6    | Tarmoq, SSH, jq, cron, Docker, CI/CD                   |

**Jami:** 16 ta bob, ~9 000 qator markdown, 60+ interaktiv mashq, **`bashlings`** CLI.

### Endi siz nima qila olasiz?

- ✅ **Server administratsiya** — SSH bilan kuniga ishlash
- ✅ **DevOps automation** — cron, systemd, CI/CD pipelines
- ✅ **API integratsiya** — curl + jq production darajada
- ✅ **Konteynerlash** — Docker bilan sandbox va deploy
- ✅ **Production skriptlar** — `set -euo pipefail`, trap, ShellCheck
- ✅ **Komanda ko'nikmasi** — `~/.ssh/config`, alias, dotfiles

### Keyingi qadamlar

1. **O'z dotfiles**'ingizni yarating — `~/.bashrc`, `~/.ssh/config`, `~/.tmux.conf` GitHub'da
2. **Real loyihaga tegishli** bo'ling — open source repo'larga PR yuboring
3. **CI/CD'ni real deploy'ga ulang** — bitta loyihangizning to'liq pipeline'ini quring
4. **Boshqa o'rgatuvchi rolga o'ting** — birovga o'rgating, eng yaxshi o'qish usuli

### Yodda saqlang

> *"Bash o'rganishning eng yaxshi yo'li — har kun real muammolarni Bash bilan hal qilishga urinish."*

Endi terminal sizniki. Foydalaning. Avtomatlashtiring. Quring.

🚀 **Yaxshi yo'l!**

---

📘 **[← Bosh sahifaga qaytish](/)**
