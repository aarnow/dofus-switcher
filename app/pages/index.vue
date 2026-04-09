<template>
  <div class="flex-1 flex flex-col overflow-hidden">

    <div class="flex items-start justify-between px-6 pt-6 pb-4 gap-4">
      <div>
        <h1 class="text-[15px] font-bold text-[#e0f5ee]">Gérer vos comptes</h1>
        <p class="text-[11px] text-[#3a7a5a] mt-1 leading-relaxed">
          Activez ou désactivez chaque compte.<br>
          Cliquez sur un compte pour l'amener au premier plan.
        </p>
      </div>
      <button
          @click="refresh"
          class="flex-shrink-0 border border-[#5DCAA5] text-[#5DCAA5] text-[11px] font-bold
               tracking-wider rounded-full px-4 py-1.5 hover:bg-[#5DCAA5] hover:text-[#0a1e1a]
               transition-all duration-150"
      >
        Actualiser
      </button>
    </div>

    <div class="flex justify-end px-6 mb-5">
      <span class="text-[13px] font-bold text-[#5DCAA5]">
        {{ enabledCount }} / {{ accounts.length }}
      </span>
    </div>

    <div class="flex-1 px-6 py-2 flex flex-col gap-3 overflow-y-auto">
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
      <EmptyState v-if="accounts.length === 0">
        Aucun client Dofus détecté
      </EmptyState>
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
  accounts.value = detected.sort((a, b) => {
    const charA = matchCharacter(a.title, characters.value)
    const charB = matchCharacter(b.title, characters.value)
    return (charB?.initiative ?? 0) - (charA?.initiative ?? 0)
  })
  activeIndex.value = 0
  await invoke('reorder_accounts', { hwnds: accounts.value.map(a => a.hwnd) })
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