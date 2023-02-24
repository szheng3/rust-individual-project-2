import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

// import './assets/main.css'
import 'vuetify/styles'
import LottieAnimation from "lottie-web-vue";


import {VueQueryPlugin} from "@tanstack/vue-query";
import {createVuetify} from "vuetify";
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const app = createApp(App)
const vuetify = createVuetify({
    components,
    directives,
})

app.use(LottieAnimation);
app.use(createPinia())
app.use(router)
app.use(VueQueryPlugin)
app.use(vuetify)

app.mount('#app')
