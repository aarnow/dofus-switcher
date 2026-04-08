<template>
  <aside class="w-48 bg-[#091820] border-r border-[#0f2a36] flex flex-col">

    <div class="px-5 py-4 border-b border-[#0f2a36]">
      <span class="text-[#e0f0f5] text-xs font-bold tracking-[3px] uppercase">
        ⚔ Switcher
      </span>
    </div>

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

    <div class="px-5 py-3 text-[10px] text-[#1a3a4a] tracking-wider">
      v1.0.0
    </div>

  </aside>
</template>

<script setup lang="ts">
const nav = [
  { to: '/',            icon: '⚔', label: 'Multi-compte' },
  { to: '/characters',  icon: '◈', label: 'Personnages'  },
  { to: '/settings',    icon: '⚙', label: 'Options'      },
]
</script>