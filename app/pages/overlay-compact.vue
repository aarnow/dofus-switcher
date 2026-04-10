<template>
  <div
      class="flex items-stretch w-full"
      style="background: rgba(0,0,0,0.85); border-radius: 4px; overflow: hidden; height: 100vh;"
  >
    <!-- Boutons gauche -->
    <div class="flex flex-col items-center justify-center gap-1 px-1.5 flex-shrink-0"
         style="background: rgba(255,255,255,0.05); width: 28px; overflow: hidden;">
      <div
          class="flex items-center justify-center rounded cursor-pointer transition-all duration-200"
          style="width: 20px; height: 20px;"
          @click="showApp"
          @mouseenter="e => (e.currentTarget as HTMLElement).style.background='rgba(93,202,165,0.3)'"
          @mouseleave="e => (e.currentTarget as HTMLElement).style.background='transparent'"
          title="Ouvrir Dofus - Heros"
      >
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="rgba(255,255,255,0.6)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
        </svg>
      </div>

      <div
          class="flex items-center justify-center rounded cursor-pointer transition-all duration-200"
          style="width: 20px; height: 20px;"
          @click="dragLocked = !dragLocked"
          @mouseenter="e => (e.currentTarget as HTMLElement).style.background='rgba(93,202,165,0.3)'"
          @mouseleave="e => (e.currentTarget as HTMLElement).style.background='transparent'"
          :title="dragLocked ? 'Déverrouiller' : 'Verrouiller'"
      >
        <svg v-if="dragLocked" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="rgba(255,200,100,0.8)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
          <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
        </svg>
        <svg v-else width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="rgba(255,255,255,0.4)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
          <path d="M7 11V7a5 5 0 0 1 9.9-1"/>
        </svg>
      </div>
    </div>

    <!-- Portraits -->
    <div class="flex flex-1 min-w-0" style="overflow: hidden;">
      <div
          v-for="(acc, i) in accounts"
          :key="acc.hwnd"
          class="flex-1 min-w-0 relative transition-all duration-200"
          :class="dragLocked ? 'cursor-pointer' : 'cursor-grab'"
          style="overflow: hidden; min-width: 0;"
          :style="i === activeIndex
          ? `border-bottom: 2px solid ${getBorderColor(acc.title)};`
          : 'border-bottom: 2px solid transparent;'"
              @mousedown="onMouseDown"
              @mouseup="onMouseUp(acc, i)"
        >
        <img
            :src="getCharacter(acc.title) && classImage(getCharacter(acc.title)!.classe)
              ? classImage(getCharacter(acc.title)!.classe)
              : '/classes/default.png'"
            :alt="getCharacter(acc.title)?.classe ?? 'default'"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; object-fit: cover;"
            :style="i !== activeIndex ? 'filter: grayscale(80%) brightness(0.5);' : ''"
        />
        <div style="position: absolute; inset: 0; background: linear-gradient(to top, rgba(0,0,0,0.8) 0%, transparent 60%);"/>
        <div style="position: absolute; bottom: 0; left: 0; right: 0; text-align: center; padding-bottom: 4px; padding-left: 4px; padding-right: 4px; font-size: 9px; font-weight: 700; letter-spacing: 0.3px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;"
             :style="i === activeIndex
               ? `color: ${getBorderColor(acc.title)};`
               : 'color: rgba(255,255,255,0.5);'"
        >
          {{ getPseudo(acc.title) }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { getCharacters, matchCharacter, classImage, classColor, extractPseudo } from '~/composables/useCharacters'
import type { Character } from '~/composables/useCharacters'
import { load } from '@tauri-apps/plugin-store'

definePageMeta({ layout: false })

interface Account {
  hwnd: number
  title: string
  enabled: boolean
}

const accounts = ref<Account[]>([])
const activeIndex = ref(0)
const characters = ref<Character[]>([])
const dragLocked = ref(true)
let mouseDownTime = 0

function onMouseDown(e: MouseEvent) {
  mouseDownTime = Date.now()
  if (!dragLocked.value) startDrag()
}

function onMouseUp(acc: Account, i: number) {
  if (dragLocked.value && Date.now() - mouseDownTime < 300) {
    focusAccount(acc.hwnd, i)
  }
}

function getCharacter(title: string): Character | null {
  return matchCharacter(title, characters.value)
}

function getPseudo(title: string): string {
  const char = getCharacter(title)
  return char ? char.pseudo : extractPseudo(title)
}

function getBorderColor(title: string): string {
  const char = getCharacter(title)
  if (!char) return '#85B7EB'
  return classColor(char.classe) || '#85B7EB'
}

async function focusAccount(hwnd: number, index: number) {
  activeIndex.value = index
  await invoke('focus_account', { hwnd })
}

async function showApp() {
  await invoke('show_main_window')
}

async function startDrag() {
  await getCurrentWindow().startDragging()
}

async function updateAccounts(raw: Account[]) {
  characters.value = await getCharacters()
  accounts.value = raw.sort((a, b) => {
    const charA = matchCharacter(a.title, characters.value)
    const charB = matchCharacter(b.title, characters.value)
    return (charB?.initiative ?? 0) - (charA?.initiative ?? 0)
  })
}

onMounted(async () => {
  characters.value = await getCharacters()

  const existing = await invoke<Account[]>('get_accounts')
  if (existing.length > 0) await updateAccounts(existing)

  await listen<Account[]>('accounts-updated', async (event) => {
    await updateAccounts(event.payload)
    activeIndex.value = 0
  })

  await listen<number>('switch', (event) => {
    const hwnd = event.payload
    const idx = accounts.value.findIndex(a => a.hwnd === hwnd)
    if (idx !== -1) activeIndex.value = idx
  })

  await listen<[number, number]>('save-overlay-position', async (event) => {
    const [x, y] = event.payload
    const store = await load('settings.json')
    await store.set('compactX', x)
    await store.set('compactY', y)
    await store.save()
  })

  // Sauvegarde la position après drag
  window.addEventListener('mouseup', async () => {
    if (!dragLocked.value) {
      const pos = await getCurrentWindow().outerPosition()
      await invoke('save_overlay_position', { x: pos.x, y: pos.y })
    }
  })
})
</script>