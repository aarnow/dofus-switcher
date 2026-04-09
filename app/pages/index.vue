<template>
  <div class="flex-1 flex flex-col overflow-hidden">

    <div class="flex items-start justify-between px-6 pt-6 pb-4 gap-4">
      <div>
        <h1 class="text-[15px] font-bold text-[#e0f0f5]">Gérer vos comptes</h1>
        <p class="text-[11px] text-[#4a7a8a] mt-1 leading-relaxed">
          Activez ou désactivez chaque compte.<br>
          Cliquez sur un compte pour l'amener au premier plan.
        </p>
      </div>
      <button
          @click="refresh"
          class="flex-shrink-0 border border-[#1eb8cc] text-[#1eb8cc] text-[11px] font-bold
               tracking-wider rounded-full px-4 py-1.5 hover:bg-[#1eb8cc] hover:text-[#091820]
               transition-all duration-150"
      >
        Actualiser
      </button>
    </div>

    <div class="flex justify-end px-6 mb-2">
      <span class="text-[13px] font-bold text-[#1eb8cc]">
        {{ enabledCount }} / {{ accounts.length }}
      </span>
    </div>

    <div class="flex-1 overflow-y-auto px-6 pb-4 flex flex-col gap-1.5">
      <AccountRow
          v-for="(acc, i) in accounts"
          :key="acc.hwnd"
          :name="acc.title"
          :index="i"
          :active="i === activeIndex"
          :enabled="acc.enabled"
          :character="getCharacterForAccount(acc.title)"
          @click="focusAccount(acc.hwnd, i)"
          @toggle="toggleAccount(acc.hwnd, !acc.enabled); acc.enabled = !acc.enabled"
      />
      <div v-if="accounts.length === 0"
           class="flex-1 flex items-center justify-center text-[#2a5060] text-sm">
        Aucun client Dofus détecté
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { getCharacters, matchCharacter } from '~/composables/useCharacters'
import type { Character } from '~/composables/useCharacters'

interface Account {
  hwnd: number
  title: string
  enabled: boolean
}

const accounts = ref<Account[]>([])
const activeIndex = ref<number>(0)
const characters = ref<Character[]>([])

const enabledCount = computed(() => accounts.value.filter(a => a.enabled).length)

function getCharacterForAccount(title: string): Character | null {
  return matchCharacter(title, characters.value)
}

async function refresh() {
  characters.value = await getCharacters()
  const detected = await invoke<Account[]>('detect_windows')

  // Trie par initiative décroissante selon le personnage associé
  accounts.value = detected.sort((a, b) => {
    const charA = matchCharacter(a.title, characters.value)
    const charB = matchCharacter(b.title, characters.value)
    const initA = charA?.initiative ?? 0
    const initB = charB?.initiative ?? 0
    return initB - initA
  })

  activeIndex.value = 0
}

async function focusAccount(hwnd: number, index: number) {
  activeIndex.value = index
  await invoke('focus_account', { hwnd })
}

async function toggleAccount(hwnd: number, enabled: boolean) {
  await invoke('toggle_account', { hwnd, enabled })
}

onMounted(async () => {
  await new Promise(resolve => setTimeout(resolve, 500))
  await refresh()

  await listen<number>('switch', (event) => {
    const hwnd = event.payload
    const idx = accounts.value.findIndex(a => a.hwnd === hwnd)
    if (idx !== -1) activeIndex.value = idx
  })
})
</script>