<template>
  <div
      class="flex items-center justify-center w-full h-full gap-2 px-2"
      style="background: transparent; overflow: hidden;"
  >
    <!-- Boutons gauche -->
    <div class="flex flex-col gap-1 items-center flex-shrink-0">

      <!-- Bouton ouvrir app -->
      <div
          class="flex items-center justify-center rounded cursor-pointer transition-all duration-200"
          style="width: 24px; height: 24px; background: rgba(0,0,0,0.5); border: 1px solid rgba(255,255,255,0.1);"
          @click="showApp"
          @mouseenter="e => (e.currentTarget as HTMLElement).style.background='rgba(93,202,165,0.3)'"
          @mouseleave="e => (e.currentTarget as HTMLElement).style.background='rgba(0,0,0,0.5)'"
          title="Ouvrir Dofus Switcher"
      >
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="rgba(255,255,255,0.6)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
        </svg>
      </div>

      <!-- Bouton verrou drag -->
      <div
          class="flex items-center justify-center rounded cursor-pointer transition-all duration-200"
          style="width: 24px; height: 24px; background: rgba(0,0,0,0.5); border: 1px solid rgba(255,255,255,0.1);"
          @click="dragLocked = !dragLocked"
          @mouseenter="e => (e.currentTarget as HTMLElement).style.background='rgba(93,202,165,0.3)'"
          @mouseleave="e => (e.currentTarget as HTMLElement).style.background='rgba(0,0,0,0.5)'"
          :title="dragLocked ? 'Déverrouiller le déplacement' : 'Verrouiller le déplacement'"
      >
        <!-- Cadenas fermé -->
        <svg v-if="dragLocked" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="rgba(255,200,100,0.8)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
          <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
        </svg>
        <!-- Cadenas ouvert -->
        <svg v-else width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="rgba(255,255,255,0.4)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
          <path d="M7 11V7a5 5 0 0 1 9.9-1"/>
        </svg>
      </div>

    </div>

    <!-- Comptes -->
    <div
        v-for="(acc, i) in accounts"
        :key="acc.hwnd"
        class="flex flex-col rounded overflow-hidden transition-all duration-200 flex-shrink-0"
        :class="dragLocked ? 'cursor-pointer' : 'cursor-grab active:cursor-grabbing'"
        style="width: 72px;"
        :style="i === activeIndex
          ? `border: 1.5px solid ${getBorderColor(acc.title)};`
          : 'border: 1.5px solid rgba(255,255,255,0.15);'"
        @click="dragLocked && focusAccount(acc.hwnd, i)"
        @mousedown="!dragLocked && startDrag()"
    >
      <div class="w-full relative" style="height: 60px;">
        <img
            :src="getCharacter(acc.title) && classImage(getCharacter(acc.title)!.classe)
              ? classImage(getCharacter(acc.title)!.classe)
              : '/classes/default.png'"
            :alt="getCharacter(acc.title)?.classe ?? 'default'"
            class="w-full h-full object-cover transition-all duration-200"
            :style="i !== activeIndex ? 'filter: grayscale(100%) brightness(0.6);' : ''"
        />
      </div>

      <div
          class="w-full text-center px-1 py-0.5 truncate"
          style="background: rgba(0,0,0,0.75); font-size: 9px; font-weight: 700; letter-spacing: 0.3px;"
          :style="i === activeIndex
            ? `color: ${getBorderColor(acc.title)};`
            : 'color: rgba(255,255,255,0.5);'"
      >
        {{ getPseudo(acc.title) }}
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

async function updateSize(count: number) {
  if (count > 0) await invoke('resize_overlay', { count })
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
  if (existing.length > 0) {
    await updateAccounts(existing)
    await updateSize(existing.length)
  }

  await listen<Account[]>('accounts-updated', async (event) => {
    await updateAccounts(event.payload)
    activeIndex.value = 0
    await updateSize(event.payload.length)
  })

  await listen<number>('switch', (event) => {
    const hwnd = event.payload
    const idx = accounts.value.findIndex(a => a.hwnd === hwnd)
    if (idx !== -1) activeIndex.value = idx
  })
})
</script>