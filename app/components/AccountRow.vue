<template>
  <div
      class="flex items-center gap-3 px-3 py-2.5 rounded-md border transition-all duration-150 cursor-pointer"
      :class="active
    ? 'bg-[#132d38] border-[#1eb8cc]'
    : 'bg-[#122530] border-[#1a3a4a] hover:border-[#2a5a6a]'"
  >
    <span class="text-[11px] font-bold font-mono text-[#2a6070] w-6 text-center">
      {{ String(index + 1).padStart(2, '0') }}
    </span>

    <div
        class="w-9 h-9 rounded-full overflow-hidden flex items-center justify-center border flex-shrink-0"
        :style="character
      ? { borderColor: classColor(character.classe) }
      : { borderColor: '#1a4560' }"
    >
      <img
          :src="character && classImage(character.classe) ? classImage(character.classe) : '/classes/default.png'"
          :alt="character ? character.classe : 'default'"
          class="w-full h-full object-cover"
      />
    </div>

    <div class="flex-1 min-w-0">
      <!-- Ligne 1 : pseudo + initiative -->
      <!-- Ligne 1 : pseudo + initiative -->
      <div class="flex items-center gap-1.5">
        <p class="text-[13px] font-bold text-[#c8e8f0] truncate">
          {{ character ? character.pseudo : name }}
        </p>
        <div v-if="character" class="flex items-center gap-0.5 flex-shrink-0
              border border-[#C268D7] rounded px-1.5 py-0.5"
             style="background: rgba(194, 104, 215, 0.12);">
          <img src="/icons/initiative.png" alt="initiative" class="w-4 h-4 object-contain" />
          <span class="text-[11px] font-bold text-[#C268D7]">{{ character.initiative }}</span>
        </div>
      </div>

      <!-- Ligne 2 : classe + éléments -->
      <div class="flex items-center gap-1.5 mt-0.5">
        <p class="text-[11px] text-[#4a7a8a]">
          {{ character ? character.classe : 'Client Dofus' }}
        </p>
        <div v-if="character" class="flex items-center gap-0.5 border border-[#1a3a4a] rounded px-1.5 py-0.5"
             style="background: rgba(0,0,0,0.3);">
          <img
              v-for="el in character.elements"
              :key="el"
              :src="elementImage(el)"
              :alt="el"
              :title="el"
              class="w-4 h-4 object-contain"
          />
        </div>
      </div>
    </div>

    <!-- Rôles -->
    <div v-if="character" class="flex gap-1 flex-wrap justify-end max-w-[80px]">
      <img
          v-for="role in character.roles.slice(0, 3)"
          :key="role"
          :src="roleImage(role)"
          :alt="role"
          :title="role"
          class="w-5 h-5 object-contain opacity-60 hover:opacity-100 transition-opacity"
      />
    </div>

    <div
        class="w-2 h-2 rounded-full flex-shrink-0"
        :class="character ? 'bg-[#1eb8cc]' : 'bg-[#1a3a4a]'"
        :title="character ? 'Rattaché à ' + character.pseudo : 'Non rattaché'"
    />

    <button
        @click.stop="$emit('toggle')"
        class="w-9 h-5 rounded-full transition-colors duration-200 flex-shrink-0 relative"
        :class="enabled ? 'bg-[#1eb8cc]' : 'bg-[#1a3a4a]'"
    >
      <span
          class="absolute top-0.5 w-4 h-4 bg-white rounded-full transition-all duration-200"
          :class="enabled ? 'right-0.5' : 'left-0.5'"
      />
    </button>
  </div>
</template>

<script setup lang="ts">
import { classColor, classImage, roleImage, elementImage } from '~/composables/useCharacters'
import type { Character } from '~/composables/useCharacters'

const props = defineProps<{
  name: string
  index: number
  active: boolean
  enabled: boolean
  character?: Character | null
}>()

defineEmits<{ toggle: [] }>()

const initials = computed(() => props.name.slice(0, 2).toUpperCase())
</script>