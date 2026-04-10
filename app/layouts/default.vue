<template>
  <div class="flex h-screen overflow-hidden select-none flex-col"
       style="background: linear-gradient(160deg, #0d2c21 0%, #0d452e 50%, #5a6737 100%);">

    <div
        data-tauri-drag-region
        class="flex items-center px-4 py-3 flex-shrink-0"
        style="background: #0c2520; border-bottom: 1px solid #1e4a38;"
    >
      <div
          data-tauri-drag-region
          class="flex items-center gap-2 pointer-events-none"
          style="min-width: 160px;"
      >
        <img src="/dofusheros.png" alt="logo" class="w-12 h-12 object-contain" />
        <div class="flex flex-col">
          <span class="text-[13px] font-bold uppercase tracking-wider" style="color: #e0f5ee;">
            Dofus - Heros
          </span>
          <span class="text-[10px] tracking-wider" style="color: #51b88e;">
            v0.1.0
          </span>
        </div>
      </div>

      <div data-tauri-drag-region class="flex-1 flex items-center justify-center">
        <div class="flex items-center gap-1 rounded-full p-1"
             style="background: rgba(0,0,0,0.25); border: 1px solid #1e4a38;">
          <NuxtLink
              v-for="item in nav"
              :key="item.to"
              :to="item.to"
              class="w-10 h-9 rounded-full flex items-center justify-center transition-all duration-150"
              style="color: #3a7a60;"
              active-class="nav-active"
              :title="item.label"
          >
            <span v-html="item.svg"
                  style="width:18px; height:18px; display:flex; align-items:center; justify-content:center;"/>
          </NuxtLink>
        </div>
      </div>

      <div data-tauri-drag-region class="flex items-center justify-end" style="min-width: 160px;">
        <button
            @click="quit"
            class="w-8 h-8 rounded-full flex items-center justify-center text-sm transition-all duration-150"
            style="background: rgba(180,60,60,0.15); border: 1px solid rgba(180,60,60,0.3); color: #cc5555;"
            @mouseenter="(e: MouseEvent) => (e.target as HTMLElement).style.background='rgba(180,60,60,0.35)'"
            @mouseleave="(e: MouseEvent) => (e.target as HTMLElement).style.background='rgba(180,60,60,0.15)'"
        >
          ✕
        </button>
      </div>
    </div>

    <main class="flex-1 flex flex-col overflow-hidden">
      <slot/>
    </main>

  </div>
</template>

<script setup lang="ts">
import {isCapturing} from '~/composables/useCapturing'
import {invoke} from '@tauri-apps/api/core'
import {load} from '@tauri-apps/plugin-store'
import {useRoute} from 'vue-router'

const route = useRoute()

const nav = [
  {
    to: '/',
    label: 'Multi-compte',
    svg: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m14 13-8.381 8.38a1 1 0 0 1-3.001-3l8.384-8.381"/><path d="m16 16 6-6"/><path d="m21.5 10.5-8-8"/><path d="m8 8 6-6"/><path d="m8.5 7.5 8 8"/></svg>`
  },
  {
    to: '/characters',
    label: 'Personnages',
    svg: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="8" r="5"/><path d="M20 21a8 8 0 0 0-16 0"/></svg>`
  },
  {
    to: '/settings',
    label: 'Options',
    svg: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"/><circle cx="12" cy="12" r="3"/></svg>`
  },
]

async function quit() {
  await invoke('quit')
}

onMounted(async () => {
  const store = await load('settings.json')

  const savedOverlay = await store.get<boolean>('overlayEnabled')
  if (savedOverlay === false) {
    await invoke('toggle_overlay', {enabled: false})
  }

  const savedVariant = await store.get<string>('overlayVariant') ?? 'default'
  const savedW = await store.get<number>('compactWidth') ?? 600
  const savedH = await store.get<number>('compactHeight') ?? 80

  const savedX = await store.get<number>('compactX') ?? -1
  const savedY = await store.get<number>('compactY') ?? 12

  await invoke('update_overlay_config', {
    variant: savedVariant,
    width: savedW,
    height: savedH,
    x: savedX,
    y: savedY,
  })

  const savedBindings = await store.get<{ id: string; value: { type: string; code: number } }[]>('bindings')
  if (savedBindings && savedBindings.length > 0) {
    const next = savedBindings.find(b => b.id === 'next')?.value
    const prev = savedBindings.find(b => b.id === 'prev')?.value
    if (next && prev) {
      await invoke('update_hotkeys', {
        keyNext: next.type === 'key' ? next.code : 0,
        keyPrev: prev.type === 'key' ? prev.code : 0,
        mouseNext: next.type === 'mouse' ? next.code : 0,
        mousePrev: prev.type === 'mouse' ? prev.code : 0,
      })
    }
  }

  window.addEventListener('mousedown', (e) => {
    if (isCapturing.value) return
    if (e.button === 3 || e.button === 4) {
      e.preventDefault();
      e.stopPropagation()
    }
  }, true)

  window.addEventListener('mouseup', (e) => {
    if (isCapturing.value) return
    if (e.button === 3 || e.button === 4) {
      e.preventDefault();
      e.stopPropagation()
    }
  }, true)

  window.addEventListener('auxclick', (e) => {
    if (isCapturing.value) return
    if (e.button === 3 || e.button === 4) {
      e.preventDefault();
      e.stopPropagation()
    }
  }, true)
})
</script>

<style>
.nav-active {
  background: linear-gradient(135deg, #5DCAA5, #2a9a75) !important;
  color: #0a1e1a !important;
  box-shadow: 0 0 10px rgba(93, 202, 165, 0.3);
}

.nav-active:hover {
  background: linear-gradient(135deg, #6dd4af, #35aa85) !important;
}
</style>