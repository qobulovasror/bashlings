<template>
  <div ref="el" class="asciinema-player-box"></div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { withBase } from 'vitepress'

// Player is loaded from a CDN at runtime (browser-only), so there is no build
// dependency and SSR is unaffected — the container renders empty on the server.
const PLAYER_VERSION = '3.8.0'
const JS_URL = `https://cdn.jsdelivr.net/npm/asciinema-player@${PLAYER_VERSION}/dist/bundle/asciinema-player.min.js`
const CSS_URL = `https://cdn.jsdelivr.net/npm/asciinema-player@${PLAYER_VERSION}/dist/bundle/asciinema-player.css`

const props = defineProps({
  src: { type: String, required: true },
  cols: { type: Number, default: 100 },
  rows: { type: Number, default: 30 },
  autoplay: { type: Boolean, default: false },
  loop: { type: Boolean, default: false },
  speed: { type: Number, default: 1.3 },
  idleTimeLimit: { type: Number, default: 2 },
  poster: { type: String, default: 'npt:0:2' }
})

const el = ref(null)
let player = null

function loadCss(href) {
  if (document.querySelector(`link[data-asciinema="1"]`)) return
  const l = document.createElement('link')
  l.rel = 'stylesheet'
  l.href = href
  l.setAttribute('data-asciinema', '1')
  document.head.appendChild(l)
}

function loadScript(src) {
  return new Promise((resolve, reject) => {
    if (window.AsciinemaPlayer) return resolve()
    const s = document.createElement('script')
    s.src = src
    s.async = true
    s.onload = resolve
    s.onerror = reject
    document.head.appendChild(s)
  })
}

onMounted(async () => {
  try {
    loadCss(CSS_URL)
    await loadScript(JS_URL)
    if (!el.value || !window.AsciinemaPlayer) return
    player = window.AsciinemaPlayer.create(withBase(props.src), el.value, {
      cols: props.cols,
      rows: props.rows,
      autoPlay: props.autoplay,
      loop: props.loop,
      speed: props.speed,
      idleTimeLimit: props.idleTimeLimit,
      poster: props.poster,
      theme: 'asciinema',
      fit: 'width'
    })
  } catch (e) {
    if (el.value) {
      el.value.innerHTML =
        '<p style="font-size:14px;opacity:.7">Terminal recording could not load (offline?). ' +
        'View the cast file directly: <a href="' + withBase(props.src) + '">' + props.src + '</a></p>'
    }
  }
})

onBeforeUnmount(() => {
  if (player && typeof player.dispose === 'function') player.dispose()
})
</script>

<style scoped>
.asciinema-player-box {
  margin: 1.5rem 0;
  border-radius: 8px;
  overflow: hidden;
}
</style>
