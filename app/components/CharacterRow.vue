<template>
  <div
      class="flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-150"
      :class="selected
        ? 'border border-[#5DCAA5]'
        : 'border border-transparent hover:border-[#2a6a4e]'"
      :style="selected
        ? 'background: rgba(93,202,165,0.12); box-shadow: 0 0 12px rgba(93,202,165,0.2);'
        : 'background: rgba(0,0,0,0.2);'"
      @click="$emit('click')"
  >
    <div
        class="w-9 h-9 rounded-full overflow-hidden flex items-center justify-center border flex-shrink-0"
        :style="{ borderColor: classColor(char.classe) || '#1a4a3a' }"
    >
      <img
          v-if="classImage(char.classe)"
          :src="classImage(char.classe)"
          :alt="char.classe"
          class="w-full h-full object-cover"
      />
      <span v-else class="text-[11px] font-bold" :style="{ color: classColor(char.classe) }">
        {{ char.pseudo.slice(0, 2).toUpperCase() }}
      </span>
    </div>

    <div class="flex-1 min-w-0">
      <p class="text-[13px] font-bold text-[#c8ead8] truncate">{{ char.pseudo }}</p>
      <p class="text-[11px] text-[#3a7a5a] truncate">{{ char.classe || 'Sans classe' }}</p>
    </div>

    <div class="flex items-center gap-0.5 flex-shrink-0 border border-[#C268D7] rounded px-1.5 py-0.5"
         style="background: rgba(194, 104, 215, 0.12);">
      <img src="/icons/initiative.png" alt="initiative" class="w-3 h-3 object-contain" />
      <span class="text-[10px] font-bold text-[#C268D7]">{{ char.initiative }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { classColor, classImage } from '~/composables/useCharacters'
import type { Character } from '~/composables/useCharacters'

defineProps<{
  char: Character
  selected?: boolean
}>()

defineEmits<{ click: [] }>()
</script>