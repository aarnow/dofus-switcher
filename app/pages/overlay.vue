<template>
  <div
      class="flex items-center justify-center w-full h-full gap-1.5"
      style="background: transparent;"
  >
    <div
        v-for="(acc, i) in accounts"
        :key="acc.hwnd"
        class="w-8 h-8 flex items-center justify-center rounded font-mono font-bold text-xs transition-all duration-200"
        :style="i === activeIndex
        ? 'background: rgba(0,0,0,0.6); border: 1.5px solid #ff6b6b; color: #ff6b6b;'
        : 'background: rgba(0,0,0,0.6); border: 1.5px solid rgba(255,255,255,0.15); color: rgba(255,255,255,0.4);'"
    >
      {{ String(i + 1).padStart(2, '0') }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

definePageMeta({ layout: false })

interface Account {
  hwnd: number
  title: string
  enabled: boolean
}

const accounts = ref<Account[]>([])
const activeIndex = ref(0)

onMounted(async () => {
  const existing = await invoke<Account[]>('get_accounts')
  if (existing.length > 0) accounts.value = existing

  await listen<Account[]>('accounts-updated', (event) => {
    accounts.value = event.payload
    activeIndex.value = 0
  })

  await listen<number>('switch', (event) => {
    activeIndex.value = event.payload
  })
})
</script>