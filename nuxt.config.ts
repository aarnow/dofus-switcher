import tailwindcss from '@tailwindcss/vite'

export default defineNuxtConfig({
    ssr: false,
    devtools: {enabled: true},
    css: ['~/assets/css/main.css'],
    vite: {
        plugins: [tailwindcss()],
    },
})
