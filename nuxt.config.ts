import tailwindcss from '@tailwindcss/vite'

export default defineNuxtConfig({
    app: {
        head: {
            link: [{ rel: 'icon', type: 'image/x-icon', href: 'data:,' }]
        }
    },
    ssr: false,
    devtools: { enabled: false },
    css: ['~/assets/css/main.css'],
    vite: {
        plugins: [tailwindcss()],
        optimizeDeps: {
            include: ['@tauri-apps/api/event']
        }
    },
})