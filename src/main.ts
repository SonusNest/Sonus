import { createApp } from "vue"
import router from './router'
import { createPinia } from 'pinia'
import { createI18n } from 'vue-i18n'
import App from "./App.vue"

import '@/assets/index.css'
import '@/assets/font.css'

import zhCn from '@/i18n/zh-cn'
import enUs from '@/i18n/en-us'

const pinia = createPinia()

const i18n = createI18n({
    legacy: false,
    globalInjection: true,
    locale: 'en-US',
    messages: {
        'zh-CN': zhCn,
        'en-US': enUs
    },
    silentTranslationWarn: true
    
})

createApp(App)
    .use(router)
    .use(pinia)
    .use(i18n)
    .mount("#app");
