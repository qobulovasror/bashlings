import DefaultTheme from 'vitepress/theme'
import AsciinemaPlayer from './AsciinemaPlayer.vue'
import './style.css'

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component('AsciinemaPlayer', AsciinemaPlayer)
  }
}
