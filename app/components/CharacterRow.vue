<template>
  <div
      class="flex items-center gap-3 px-4 py-3 bg-[#122530] border border-[#1a3a4a]
             rounded-md hover:border-[#2a5a6a] transition-colors cursor-pointer"
      @click="$emit('click')"
  >
    <div
        class="w-9 h-9 rounded-full overflow-hidden flex items-center justify-center border flex-shrink-0"
        :style="{ borderColor: classColor(char.classe) }"
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
      <div class="flex items-center gap-1.5">
        <p class="text-[13px] font-bold text-[#c8e8f0] truncate">{{ char.pseudo }}</p>
        <div class="flex items-center gap-0.5 flex-shrink-0 border border-[#C268D7] rounded px-1.5 py-0.5"
             style="background: rgba(194, 104, 215, 0.12);">
          <img src="/icons/initiative.png" alt="initiative" class="w-4 h-4 object-contain" />
          <span class="text-[11px] font-bold text-[#C268D7]">{{ char.initiative }}</span>
        </div>
      </div>

      <div class="flex items-center gap-1.5 mt-0.5">
        <p class="text-[11px] text-[#4a7a8a]">{{ char.classe }}</p>
        <div v-if="char.elements.length" class="flex items-center gap-0.5 border border-[#1a3a4a] rounded px-1.5 py-0.5"
             style="background: rgba(0,0,0,0.3);">
          <img
              v-for="el in char.elements"
              :key="el"
              :src="elementImage(el)"
              :alt="el"
              :title="el"
              class="w-4 h-4 object-contain"
          />
        </div>
      </div>
    </div>

    <div class="flex gap-1 flex-wrap justify-end max-w-[140px]">
      <img
          v-for="role in char.roles"
          :key="role"
          :src="roleImage(role)"
          :alt="role"
          :title="role"
          class="w-5 h-5 object-contain opacity-60 hover:opacity-100 transition-opacity"
      />
    </div>

    <template v-if="deletable">
      <button
          v-if="!confirming"
          @click.stop="confirming = true"
          class="text-[#2a5060] hover:text-[#ff6b6b] transition-colors text-sm ml-1 flex-shrink-0"
      >
        ✕
      </button>

      <div v-else class="flex items-center gap-1.5 ml-1 flex-shrink-0" @click.stop>
        <span class="text-[10px] text-[#ff6b6b] font-bold">Supprimer ?</span>
        <button
            @click.stop="$emit('delete'); confirming = false"
            class="text-[9px] font-bold px-1.5 py-0.5 rounded bg-[#ff6b6b22]
                   border border-[#ff6b6b66] text-[#ff6b6b] hover:bg-[#ff6b6b44] transition-colors"
        >
          Oui
        </button>
        <button
            @click.stop="confirming = false"
            class="text-[9px] font-bold px-1.5 py-0.5 rounded bg-[#1a3a4a]
                   border border-[#2a5a6a] text-[#4a7a8a] hover:text-[#e0f0f5] transition-colors"
        >
          Non
        </button>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { classColor, classImage, roleImage, elementImage } from '~/composables/useCharacters'
import type { Character } from '~/composables/useCharacters'

defineProps<{
  char: Character
  deletable?: boolean
}>()

defineEmits<{ click: [], delete: [] }>()

const confirming = ref(false)
</script>