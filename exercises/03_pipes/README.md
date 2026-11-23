# 03 — I/O Redirection va Pipelines

📘 **Kitob bobi:** [`docs/part1/03-pipes-redirection.md`](../../docs/part1/03-pipes-redirection.md)

## Mashqlar ro'yxati

| # | Nomi    | Mavzu                                  | Daraja      |
|---|---------|----------------------------------------|-------------|
| 1 | `pipe1` | `>` — faylga yo'naltirish              | ★☆☆☆☆       |
| 2 | `pipe2` | `>>` — faylga qo'shib yozish           | ★★☆☆☆       |
| 3 | `pipe3` | `\|` + `tr` — bo'shliqni newline'ga    | ★★☆☆☆       |
| 4 | `pipe4` | `seq \| tail` — oxirgi N elementlar    | ★★☆☆☆       |
| 5 | `pipe5` | `grep -c` — moslashlar sonini sanash   | ★★★☆☆       |
| 6 | `pipe6` | `sort \| uniq` — takrorlarsiz tartibga | ★★★☆☆       |
| 7 | `pipe7` | `tee` — ikkala tomonga yozish          | ★★★☆☆       |
| 8 | `pipe8` | `<<<` — here-string bilan input        | ★★★☆☆       |

## Sandbox

Har mashq bir martalik sandbox ichida (`.bashlings/sandbox/<nom>-…/work/`) ishlaydi — sizning workspace'ingiz ifloslanmaydi, run tugagach katalog o'chiriladi.

## Boshlash

```bash
bashlings watch
```
