<template>
  <div
      class="flex items-center justify-center w-full h-full gap-2 px-2"
      style="background: transparent;"
  >
    <div
        v-for="(acc, i) in accounts"
        :key="acc.hwnd"
        class="flex flex-col rounded overflow-hidden transition-all duration-200 flex-shrink-0"
        style="width: 72px;"
        :style="i === activeIndex
          ? `border: 1.5px solid ${getBorderColor(acc.title)};`
          : 'border: 1.5px solid rgba(255,255,255,0.15);'"
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

async function updateSize(count: number) {
  if (count > 0) await invoke('resize_overlay', { count })
}

onMounted(async () => {
  characters.value = await getCharacters()

  const existing = await invoke<Account[]>('get_accounts')
  if (existing.length > 0) {
    accounts.value = existing
    await updateSize(existing.length)
  }

  await listen<Account[]>('accounts-updated', async (event) => {
    characters.value = await getCharacters()
    accounts.value = event.payload
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