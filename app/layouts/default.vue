<template>
  <div class="flex h-screen bg-[#0d1f26] overflow-hidden select-none">
    <AppSidebar />
    <main class="flex-1 flex flex-col overflow-hidden">
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
import { isCapturing } from '~/composables/useCapturing'
import { invoke } from '@tauri-apps/api/core'
import { load } from '@tauri-apps/plugin-store'

onMounted(async () => {
  // Charge et applique les préférences au démarrage
  const store = await load('settings.json')

  // Overlay
  const savedOverlay = await store.get<boolean>('overlayEnabled')
  if (savedOverlay === false) {
    await invoke('toggle_overlay', { enabled: false })
  }

  // Hotkeys
  const savedBindings = await store.get<{ id: string; value: { type: string; code: number } }[]>('bindings')
  if (savedBindings && savedBindings.length > 0) {
    const next = savedBindings.find(b => b.id === 'next')?.value
    const prev = savedBindings.find(b => b.id === 'prev')?.value
    if (next && prev) {
      await invoke('update_hotkeys', {
        keyNext:   next.type === 'key'   ? next.code : 0,
        keyPrev:   prev.type === 'key'   ? prev.code : 0,
        mouseNext: next.type === 'mouse' ? next.code : 0,
        mousePrev: prev.type === 'mouse' ? prev.code : 0,
      })
    }
  }

  // Blocage Mouse4/Mouse5
  window.addEventListener('mousedown', (e) => {
    if (isCapturing.value) return
    if (e.button === 3 || e.button === 4) {
      e.preventDefault()
      e.stopPropagation()
    }
  }, true)

  window.addEventListener('mouseup', (e) => {
    if (isCapturing.value) return
    if (e.button === 3 || e.button === 4) {
      e.preventDefault()
      e.stopPropagation()
    }
  }, true)

  window.addEventListener('auxclick', (e) => {
    if (isCapturing.value) return
    if (e.button === 3 || e.button === 4) {
      e.preventDefault()
      e.stopPropagation()
    }
  }, true)
})
</script>