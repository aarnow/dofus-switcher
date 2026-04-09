<template>
  <aside class="w-48 bg-[#091820] border-r border-[#0f2a36] flex flex-col">

    <nav class="flex-1 py-3">
      <NuxtLink
          v-for="item in nav"
          :key="item.to"
          :to="item.to"
          class="flex items-center gap-2 px-5 py-3 text-[11px] font-bold tracking-widest uppercase
               text-[#4a7a8a] border-l-[3px] border-transparent
               hover:text-[#a0d0dc] hover:bg-[#0f2530] transition-all duration-150"
          active-class="text-[#e0f0f5] bg-[#0f2a36] border-l-[#1eb8cc]"
      >
        <span class="text-base leading-none">{{ item.icon }}</span>
        {{ item.label }}
      </NuxtLink>
    </nav>

    <button
        @click="quit"
        class="w-full flex items-center justify-center gap-2 py-3 text-[11px] font-bold
             tracking-widest uppercase border-t border-[#0f2a36]
             bg-[#3a1010] text-[#ff6b6b] hover:bg-[#4a1515] hover:text-[#ff9090]
             transition-all duration-150"
    >
      <span class="text-base leading-none">⏻</span>
      Quitter
    </button>

  </aside>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'

const nav = [
  { to: '/',           icon: '⚔', label: 'Multi-compte' },
  { to: '/characters', icon: '◈', label: 'Personnages'  },
  { to: '/settings',   icon: '⚙', label: 'Options'      },
]

async function quit() {
  await invoke('quit')
}
</script>