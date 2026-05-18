import { defineConfig } from 'vitepress'

export default defineConfig({
  lang: 'uz-UZ',
  title: 'Bash & Linux Darsligi',
  description: "Linux buyruqlari va Advanced Bash Scripting bo'yicha o'zbek tilidagi to'liq qo'llanma",

  lastUpdated: true,
  cleanUrls: true,

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
    logo: { src: '/logo.svg', width: 28, height: 28 },
    siteTitle: 'Bash UZ',

    nav: [
      { text: 'Bosh sahifa', link: '/' },
      {
        text: "Darslar",
        items: [
          { text: "1-qism: Linux & Bash Asoslari", link: '/part1/01-introduction' },
          { text: "2-qism: Advanced Scripting", link: '/part2/01-functions' },
          { text: "3-qism: Real-world ko'nikmalar", link: '/part3/01-network' }
        ]
      },
      { text: 'Manbalar', link: '/resources' }
    ],

    sidebar: {
      '/part1/': [
        {
          text: "1-QISM · Linux Buyruqlari va Bash Asoslari",
          collapsed: false,
          items: [
            { text: '1. Shell, Terminal va Bash nima?', link: '/part1/01-introduction' },
            { text: '2. Fayl tizimi va navigatsiya', link: '/part1/02-navigation' },
            { text: "3. I/O Redirection va Pipelines", link: '/part1/03-pipes-redirection' },
            { text: "4. Matnlar bilan ishlash", link: '/part1/04-text-processing' },
            { text: "5. Birinchi Bash skript", link: '/part1/05-basic-scripting' }
          ]
        }
      ],
      '/part2/': [
        {
          text: "2-QISM · Advanced Bash Scripting",
          collapsed: false,
          items: [
            { text: "1. Funksiyalar va modullik", link: '/part2/01-functions' },
            { text: "2. Massivlar va lug'atlar", link: '/part2/02-arrays' },
            { text: "3. sed, awk va grep mahorat", link: '/part2/03-sed-awk' },
            { text: "4. Signallar va traps", link: '/part2/04-traps-signals' },
            { text: "5. Robust skriptlar (set -euo pipefail)", link: '/part2/05-robust-scripting' }
          ]
        }
      ],
      '/part3/': [
        {
          text: "3-QISM · Real-world ko'nikmalar",
          collapsed: false,
          items: [
            { text: "1. Tarmoq buyruqlari (curl, ping, wget)", link: '/part3/01-network' },
            { text: "2. SSH va remote management", link: '/part3/02-ssh' },
            { text: "3. JSON va YAML — jq, yq", link: '/part3/03-jq' },
            { text: "4. Cron va vazifalarni rejalashtirish", link: '/part3/04-cron' },
            { text: "5. Docker bilan integratsiya", link: '/part3/05-docker' },
            { text: "6. CI/CD — GitHub Actions", link: '/part3/06-cicd' }
          ]
        }
      ]
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/' }
    ],

    search: {
      provider: 'local',
      options: {
        locales: {
          root: {
            translations: {
              button: {
                buttonText: 'Qidirish',
                buttonAriaLabel: 'Qidirish'
              },
              modal: {
                noResultsText: 'Hech narsa topilmadi',
                resetButtonTitle: 'Tozalash',
                footer: {
                  selectText: 'tanlash',
                  navigateText: "harakatlanish",
                  closeText: 'yopish'
                }
              }
            }
          }
        }
      }
    },

    outline: {
      level: [2, 3],
      label: 'Sahifa mundarijasi'
    },

    docFooter: {
      prev: 'Oldingi sahifa',
      next: 'Keyingi sahifa'
    },

    editLink: {
      pattern: 'https://github.com/qobulovasror/bash-doc-uz/edit/main/docs/:path',
      text: 'Bu sahifani tahrirlash'
    },

    lastUpdated: {
      text: "Oxirgi yangilanish",
      formatOptions: { dateStyle: 'long', timeStyle: 'short' }
    },

    footer: {
      message: 'MIT litsenziyasi asosida tarqatiladi.',
      copyright: '© 2026 Bash UZ Darsligi'
    },

    returnToTopLabel: 'Tepaga qaytish',
    sidebarMenuLabel: 'Menyu',
    darkModeSwitchLabel: 'Tema',
    lightModeSwitchTitle: 'Yorug rejimga o\'tish',
    darkModeSwitchTitle: 'Qorong\'u rejimga o\'tish'
  },

  markdown: {
    theme: { light: 'github-light', dark: 'github-dark' },
    lineNumbers: true,
    container: {
      tipLabel: 'MASLAHAT',
      warningLabel: 'OGOHLANTIRISH',
      dangerLabel: 'XAVF',
      infoLabel: "MA'LUMOT",
      detailsLabel: "Batafsil"
    }
  }
})
