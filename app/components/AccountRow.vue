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
          v-if="character && classImage(character.classe)"
          :src="classImage(character.classe)"
          :alt="character.classe"
          class="w-full h-full object-cover"
      />
      <span v-else class="text-[11px] font-bold text-[#5DCAA5]">
        {{ initials }}
      </span>
    </div>

    <div class="flex-1 min-w-0">
      <p class="text-[13px] font-bold text-[#c8e8f0] truncate">
        {{ character ? character.pseudo : name }}
      </p>
      <p class="text-[11px] text-[#4a7a8a]">
        {{ character ? character.classe + ' · Init. ' + character.initiative : 'Client Dofus' }}
      </p>
    </div>

    <div v-if="character" class="flex gap-1 flex-wrap justify-end max-w-[120px]">
      <span v-for="role in character.roles.slice(0, 2)" :key="role"
            class="text-[9px] font-bold px-1.5 py-0.5 rounded bg-[#0f2a36]
                   text-[#1eb8cc] border border-[#1eb8cc22]">
        {{ role }}
      </span>
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
import { classColor, classImage } from '~/composables/useCharacters'
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