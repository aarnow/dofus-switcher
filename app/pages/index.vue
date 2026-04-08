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
          @click="focusAccount(acc.hwnd, i)"
          @toggle="toggleAccount(acc.hwnd, !acc.enabled); acc.enabled = !acc.enabled"
      />
      <div v-if="accounts.length === 0"
           class="flex-1 flex items-center justify-center text-[#2a5060] text-sm">
        Aucun client Dofus détecté
      </div>
    </div>

    <div class="border-t border-[#0f2a36] bg-[#091820] px-6 py-3 flex items-center gap-2">
      <span class="text-[#1eb8cc] text-[10px]">●</span>
      <span class="text-[11px] text-[#2a6070]">En écoute · Mouse4 / Mouse5 actifs</span>
      <div class="ml-auto">
        <button
            class="bg-[#1eb8cc] text-[#091820] font-bold text-[12px] tracking-widest
                 rounded-full px-8 py-2 hover:bg-[#28d4e8] transition-colors"
        >
          JOUER
        </button>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

interface Account {
  hwnd: number
  title: string
  enabled: boolean
}

const accounts = ref<Account[]>([])
const activeIndex = ref<number>(0)
const enabledCount = computed(() => accounts.value.filter(a => a.enabled).length)

async function refresh() {
  accounts.value = await invoke<Account[]>('detect_windows')
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
  await refresh()
  await listen<number>('switch', (event) => {
    activeIndex.value = event.payload
  })
})
</script>