import { defineConfig } from 'vitepress'

export default defineConfig({
  lastUpdated: true,
  cleanUrls: true,

  // Faza 4: inglizcha tarjima bosqichma-bosqich qo'shilmoqda. Hamma /en/ sahifalari
  // tayyor bo'lgach, bu qator olib tashlanadi (qat'iy dead-link tekshiruvi uchun).
  ignoreDeadLinks: true,

  head: [
    ['link', { rel: 'icon', href: '/favicon.svg', type: 'image/svg+xml' }],
    ['meta', { name: 'theme-color', content: '#3eaf7c' }],
    ['meta', { property: 'og:title', content: 'Bash & Linux Darsligi' }],
    ['meta', {
      property: 'og:description',
      content: "Linux va Bash bo'yicha o'zbek tilidagi interaktiv darslik"
    }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { name: 'viewport', content: 'width=device-width, initial-scale=1.0' }]
  ],

  themeConfig: {
    // ── Locale'lar uchun umumiy ──────────────────────────────────────────
    logo: { src: '/logo.svg', width: 28, height: 28 },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/qobulovasror/bashlings' }
    ],

    search: {
      provider: 'local',
      options: {
        locales: {
          root: {
            translations: {
              button: { buttonText: 'Qidirish', buttonAriaLabel: 'Qidirish' },
              modal: {
                noResultsText: 'Hech narsa topilmadi',
                resetButtonTitle: 'Tozalash',
                footer: { selectText: 'tanlash', navigateText: 'harakatlanish', closeText: 'yopish' }
              }
            }
          }
        }
      }
    },

    markdown: {
      theme: { light: 'github-light', dark: 'github-dark' }
    }
  },

  // ── Tillar ─────────────────────────────────────────────────────────────
  locales: {
    root: {
      label: "O'zbekcha",
      lang: 'uz-UZ',
      title: 'Bash & Linux Darsligi',
      description: "Linux buyruqlari va Advanced Bash Scripting bo'yicha o'zbek tilidagi to'liq qo'llanma",
      themeConfig: {
        siteTitle: 'Bash UZ',
        nav: [
          { text: 'Bosh sahifa', link: '/' },
          {
            text: 'Boshlash',
            items: [
              { text: "Kirish so'zi", link: '/foreword' },
              { text: "O'rnatish", link: '/setup' },
              { text: "Atamalar lug'ati", link: '/glossary' }
            ]
          },
          {
            text: 'Darslar',
            items: [
              { text: '1-qism: Linux & Bash Asoslari', link: '/part1/01-introduction' },
              { text: '2-qism: Advanced Scripting', link: '/part2/01-functions' },
              { text: "3-qism: Real-world ko'nikmalar", link: '/part3/01-network' }
            ]
          },
          { text: 'Manbalar', link: '/resources' }
        ],
        sidebar: {
          '/foreword': [
            {
              text: 'BOSHLASH',
              collapsed: false,
              items: [
                { text: "Kirish so'zi", link: '/foreword' },
                { text: "O'rnatish va sozlash", link: '/setup' },
                { text: "Atamalar lug'ati", link: '/glossary' }
              ]
            }
          ],
          '/setup': [
            {
              text: 'BOSHLASH',
              collapsed: false,
              items: [
                { text: "Kirish so'zi", link: '/foreword' },
                { text: "O'rnatish va sozlash", link: '/setup' },
                { text: "Atamalar lug'ati", link: '/glossary' }
              ]
            }
          ],
          '/glossary': [
            {
              text: 'BOSHLASH',
              collapsed: false,
              items: [
                { text: "Kirish so'zi", link: '/foreword' },
                { text: "O'rnatish va sozlash", link: '/setup' },
                { text: "Atamalar lug'ati", link: '/glossary' }
              ]
            }
          ],
          '/part1/': [
            {
              text: '1-QISM · Linux Buyruqlari va Bash Asoslari',
              collapsed: false,
              items: [
                { text: '1. Shell, Terminal va Bash nima?', link: '/part1/01-introduction' },
                { text: '2. Fayl tizimi va navigatsiya', link: '/part1/02-navigation' },
                { text: '3. I/O Redirection va Pipelines', link: '/part1/03-pipes-redirection' },
                { text: '4. Matnlar bilan ishlash', link: '/part1/04-text-processing' },
                { text: '5. Birinchi Bash skript', link: '/part1/05-basic-scripting' }
              ]
            }
          ],
          '/part2/': [
            {
              text: '2-QISM · Advanced Bash Scripting',
              collapsed: false,
              items: [
                { text: '1. Funksiyalar va modullik', link: '/part2/01-functions' },
                { text: "2. Massivlar va lug'atlar", link: '/part2/02-arrays' },
                { text: '3. sed, awk va grep mahorat', link: '/part2/03-sed-awk' },
                { text: '4. Signallar va traps', link: '/part2/04-traps-signals' },
                { text: '5. Robust skriptlar (set -euo pipefail)', link: '/part2/05-robust-scripting' }
              ]
            }
          ],
          '/part3/': [
            {
              text: "3-QISM · Real-world ko'nikmalar",
              collapsed: false,
              items: [
                { text: '1. Tarmoq buyruqlari (curl, ping, wget)', link: '/part3/01-network' },
                { text: '2. SSH va remote management', link: '/part3/02-ssh' },
                { text: '3. JSON va YAML — jq, yq', link: '/part3/03-jq' },
                { text: '4. Cron va vazifalarni rejalashtirish', link: '/part3/04-cron' },
                { text: '5. Docker bilan integratsiya', link: '/part3/05-docker' },
                { text: '6. CI/CD — GitHub Actions', link: '/part3/06-cicd' }
              ]
            }
          ]
        },
        outline: { level: [2, 3], label: 'Sahifa mundarijasi' },
        docFooter: { prev: 'Oldingi sahifa', next: 'Keyingi sahifa' },
        editLink: {
          pattern: 'https://github.com/qobulovasror/bashlings/edit/main/docs/:path',
          text: 'Bu sahifani tahrirlash'
        },
        lastUpdated: {
          text: 'Oxirgi yangilanish',
          formatOptions: { dateStyle: 'long', timeStyle: 'short' }
        },
        footer: {
          message: 'MIT litsenziyasi asosida tarqatiladi.',
          copyright: '© 2026 Bash UZ Darsligi'
        },
        returnToTopLabel: 'Tepaga qaytish',
        sidebarMenuLabel: 'Menyu',
        darkModeSwitchLabel: 'Tema',
        lightModeSwitchTitle: "Yorug' rejimga o'tish",
        darkModeSwitchTitle: "Qorong'u rejimga o'tish"
      }
    },

    en: {
      label: 'English',
      lang: 'en-US',
      title: 'Bash & Linux Guide',
      description: 'A complete guide to Linux commands and advanced Bash scripting (Uzbek-first, English translation).',
      themeConfig: {
        siteTitle: 'Bash UZ',
        nav: [
          { text: 'Home', link: '/en/' },
          {
            text: 'Getting started',
            items: [
              { text: 'Foreword', link: '/en/foreword' },
              { text: 'Setup', link: '/en/setup' },
              { text: 'Glossary', link: '/en/glossary' }
            ]
          },
          {
            text: 'Lessons',
            items: [
              { text: 'Part 1: Linux & Bash Basics', link: '/en/part1/01-introduction' },
              { text: 'Part 2: Advanced Scripting', link: '/en/part2/01-functions' }
            ]
          },
          { text: 'Resources', link: '/en/resources' }
        ],
        sidebar: {
          '/en/foreword': [
            {
              text: 'GETTING STARTED',
              collapsed: false,
              items: [
                { text: 'Foreword', link: '/en/foreword' },
                { text: 'Setup & configuration', link: '/en/setup' },
                { text: 'Glossary', link: '/en/glossary' }
              ]
            }
          ],
          '/en/setup': [
            {
              text: 'GETTING STARTED',
              collapsed: false,
              items: [
                { text: 'Foreword', link: '/en/foreword' },
                { text: 'Setup & configuration', link: '/en/setup' },
                { text: 'Glossary', link: '/en/glossary' }
              ]
            }
          ],
          '/en/glossary': [
            {
              text: 'GETTING STARTED',
              collapsed: false,
              items: [
                { text: 'Foreword', link: '/en/foreword' },
                { text: 'Setup & configuration', link: '/en/setup' },
                { text: 'Glossary', link: '/en/glossary' }
              ]
            }
          ],
          '/en/part1/': [
            {
              text: 'PART 1 · Linux Commands & Bash Basics',
              collapsed: false,
              items: [
                { text: '1. What are the shell, terminal, and Bash?', link: '/en/part1/01-introduction' },
                { text: '2. Filesystem and navigation', link: '/en/part1/02-navigation' },
                { text: '3. I/O redirection and pipelines', link: '/en/part1/03-pipes-redirection' },
                { text: '4. Working with text', link: '/en/part1/04-text-processing' },
                { text: '5. Your first Bash script', link: '/en/part1/05-basic-scripting' }
              ]
            }
          ],
          '/en/part2/': [
            {
              text: 'PART 2 · Advanced Bash Scripting',
              collapsed: false,
              items: [
                { text: '1. Functions and modularity', link: '/en/part2/01-functions' },
                { text: '2. Arrays and associative arrays', link: '/en/part2/02-arrays' },
                { text: '3. sed, awk, and grep mastery', link: '/en/part2/03-sed-awk' },
                { text: '4. Signals and traps', link: '/en/part2/04-traps-signals' },
                { text: '5. Robust scripts (set -euo pipefail)', link: '/en/part2/05-robust-scripting' }
              ]
            }
          ]
          // NOTE: Part 3 sidebar added when those pages are translated (step 4.4).
        },
        outline: { level: [2, 3], label: 'On this page' },
        docFooter: { prev: 'Previous', next: 'Next' },
        editLink: {
          pattern: 'https://github.com/qobulovasror/bashlings/edit/main/docs/:path',
          text: 'Edit this page'
        },
        lastUpdated: {
          text: 'Last updated',
          formatOptions: { dateStyle: 'long', timeStyle: 'short' }
        },
        footer: {
          message: 'Released under the MIT License.',
          copyright: '© 2026 Bash UZ'
        }
      }
    }
  },

  markdown: {
    theme: { light: 'github-light', dark: 'github-dark' },
    lineNumbers: true,
    container: {
      tipLabel: 'MASLAHAT',
      warningLabel: 'OGOHLANTIRISH',
      dangerLabel: 'XAVF',
      infoLabel: "MA'LUMOT",
      detailsLabel: 'Batafsil'
    }
  }
})
