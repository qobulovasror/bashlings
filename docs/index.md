---
layout: home

hero:
  name: "Bash & Linux"
  text: "O'zbek tilidagi to'liq darslik"
  tagline: "Terminalni o'rganing, Linux ekotizimini o'zlashtirib oling va kuchli Bash skriptlari yozing — boshlovchidan professionalgacha."
  image:
    src: /hero.svg
    alt: Bash UZ
  actions:
    - theme: brand
      text: "Boshlash →"
      link: /foreword
    - theme: alt
      text: "O'rnatish"
      link: /setup
    - theme: alt
      text: "GitHub"
      link: https://github.com/qobulovasror/bashlings

features:
  - icon: 🐧
    title: "Linux asoslari"
    details: "Terminal, shell va fayl tizimi bilan tanishib chiqing. Har bir buyruq amaliy misol bilan tushuntiriladi."
  - icon: ⚡
    title: "Tezkor amaliyot"
    details: "Har bir bobning oxirida real terminal misollari, mashqlar va xatolar tahlili keltirilgan."
  - icon: 🔧
    title: "Quvurlar va qayta yo'naltirish"
    details: "`|`, `>`, `>>`, `<`, `tee` — buyruqlarni bir-biri bilan bog'lab, kuchli pipeline yaratishni o'rganamiz."
  - icon: 📜
    title: "Bash Scripting"
    details: "O'zgaruvchilar, shartlar, looplar, funksiyalar va massivlar — barchasi tartibli va mantiqiy ketma-ketlikda."
  - icon: 🛡️
    title: "Robust skriptlar"
    details: "`set -euo pipefail`, traps, xatoliklarni boshqarish — production-ready skriptlar yozish ko'nikmasini olasiz."
  - icon: 🎯
    title: "Real loyihalarga tayyor"
    details: "Darslik so'ngida CI/CD, DevOps va serveradministratsiya vazifalarini bemalol bajara olasiz."
---

<div style="max-width: 960px; margin: 4rem auto 0; padding: 0 24px;">

## Nima uchun bu darslik?

Ko'pchilik o'zbek dasturchilari uchun terminal va Bash hali ham "sirli" hudud bo'lib qoladi. Bu darslik aynan shu bo'shliqni to'ldirishga qaratilgan — biz **nazariya + amaliyot + xatolar tahlili** uch ustunini birlashtirib bordik.

::: tip Kim uchun mo'ljallangan?
- Linuxni endi o'rganayotgan **boshlovchilar**
- DevOps yo'liga kirayotgan **junior dasturchilar**
- Server administratsiyasi bilan ishlovchi **muhandislar**
- O'z workflow'larini avtomatlashtirishni xohlovchi har kim
:::

## Darslik tuzilishi

Darslik **3 ta katta qism**dan iborat — jami **16 bob** va **101 interaktiv mashq**.

### 📘 1-qism — Linux & Bash asoslari (32 mashq)
Boshlovchilar uchun mo'ljallangan. Terminaldan tortib, birinchi Bash skript yozishgacha bo'lgan yo'l.

[**→ 1-qismni boshlash**](/part1/01-introduction)

### 📗 2-qism — Advanced Bash Scripting (28 mashq)
Funksiyalar, massivlar, `sed`/`awk`, signallar va production-grade skriptlar.

[**→ 2-qismga o'tish**](/part2/01-functions)

### 📕 3-qism — Real-world ko'nikmalar (41 mashq)
`curl`, SSH, `jq`, cron, Docker, GitHub Actions — DevOps yo'lidagi asosiy ko'nikmalar.

[**→ 3-qismga o'tish**](/part3/01-network)

---

## Yo'l xaritasi

::: tip Yangi foydalanuvchimisiz?
**Tartib bo'yicha:**
1. [**Kirish so'zi**](/foreword) — uchta ustun falsafasi, kim uchun mo'ljallangan
2. [**O'rnatish**](/setup) — Bash 4+, Rust va `bashlings` CLI'ni 15 daqiqada sozlash
3. [**1-qism, 1-bob**](/part1/01-introduction) — birinchi terminal qadami
4. Har bobdan keyin: terminalda `bashlings watch`

**Foydali sahifalar:**
- [**Atamalar lug'ati**](/glossary) — uchragan har bir atama izohi
- [**Manbalar**](/resources) — qo'shimcha o'qish va vositalar
:::

---

## Manbalar

Darslik quyidagi mashhur ochiq manbalarning eng yaxshi amaliyotlaridan ilhomlangan:

- [`jlevy/the-art-of-command-line`](https://github.com/jlevy/the-art-of-command-line)
- [`denysdovhan/bash-handbook`](https://github.com/denysdovhan/bash-handbook)
- [GNU Bash Reference Manual](https://www.gnu.org/software/bash/manual/)

</div>
