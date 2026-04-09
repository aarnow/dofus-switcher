<template>
  <div
      class="flex items-center rounded-xl transition-all duration-200 cursor-pointer overflow-hidden"
      :style="active
        ? 'background: rgba(0,0,0,0.35); box-shadow: 0 0 20px rgba(93,202,165,0.65), 0 0 0 1px rgba(93,202,165,0.5);'
        : 'background: rgba(0,0,0,0.35);'"
  >
    <!-- Index -->
    <div class="flex items-center justify-center flex-shrink-0 self-stretch px-3"
         :style="active ? 'background: rgba(93,202,165,0.2);' : 'background: rgba(0,0,0,0.2);'">
      <span class="text-[13px] font-bold font-mono"
            :style="active ? 'color: #5DCAA5;' : 'color: #2a5a3a;'">
        {{ index + 1 }}
      </span>
    </div>

    <!-- Avatar -->
    <div class="px-3 py-2.5 flex-shrink-0">
      <div class="w-11 h-11 rounded-full overflow-hidden border-2 flex-shrink-0"
           :style="character ? { borderColor: classColor(character.classe) } : { borderColor: '#1a3a2e' }">
        <img
            :src="character && classImage(character.classe) ? classImage(character.classe) : '/classes/default.png'"
            :alt="character ? character.classe : 'default'"
            class="w-full h-full object-cover"
        />
      </div>
    </div>

    <AccountRowSeparator />

    <!-- Pseudo + Classe -->
    <div class="flex flex-col justify-center px-4 py-2.5" style="width: 130px; flex-shrink: 0;">
      <p class="text-[13px] font-bold text-[#c8ead8] truncate leading-tight">
        {{ character ? character.pseudo : name }}
      </p>
      <p class="text-[10px] text-[#3a7a5a] truncate mt-0.5">
        {{ character ? character.classe : 'Client Dofus' }}
      </p>
    </div>

    <AccountRowSeparator />

    <!-- Initiative -->
    <div class="flex items-center justify-center px-4 flex-shrink-0" style="width: 72px;">
      <div v-if="character" class="flex flex-col items-center gap-0.5">
        <img src="/icons/initiative.png" alt="initiative" class="w-5 h-5 object-contain opacity-70" />
        <span class="text-[12px] font-bold text-[#C268D7]">{{ character.initiative }}</span>
      </div>
      <span v-else class="text-[#1a3a2e] text-lg">—</span>
    </div>

    <AccountRowSeparator />

    <!-- Éléments -->
    <div class="flex items-center justify-center px-3 flex-shrink-0" style="width: 100px;">
      <div v-if="character && character.elements.length" class="flex flex-wrap gap-1 justify-center">
        <img v-for="el in character.elements" :key="el"
             :src="elementImage(el)" :alt="el" :title="el"
             class="w-5 h-5 object-contain" />
      </div>
      <span v-else class="text-[#1a3a2e] text-lg">—</span>
    </div>

    <AccountRowSeparator />

    <!-- Rôles -->
    <div class="flex items-center justify-center px-3 flex-shrink-0" style="width: 160px;">
      <div v-if="character && character.roles.length" class="flex flex-wrap gap-1 justify-center">
        <img v-for="role in character.roles" :key="role"
             :src="roleImage(role)" :alt="role" :title="role"
             class="w-5 h-5 object-contain transition-opacity" />
      </div>
      <span v-else class="text-[#1a3a2e] text-lg">—</span>
    </div>

    <div class="flex-1" />

    <AccountRowSeparator />

    <!-- Indicateurs -->
    <div class="flex items-center gap-3 px-4 flex-shrink-0">
      <div
          class="w-1.5 h-1.5 rounded-full"
          :class="character ? 'bg-[#5DCAA5]' : 'bg-[#1a3a2e]'"
          :title="character ? 'Rattaché' : 'Non rattaché'"
      />
      <AppToggle
          :model-value="enabled"
          @update:model-value="$emit('toggle')"
          @click.stop
      />
    </div>

  </div>
</template>

<script setup lang="ts">
import { classColor, classImage, roleImage, elementImage } from '~/composables/useCharacters'
import type { Character } from '~/composables/useCharacters'

defineProps<{
  name: string
  index: number
  active: boolean
  enabled: boolean
  character?: Character | null
}>()

defineEmits<{ toggle: [] }>()
</script>