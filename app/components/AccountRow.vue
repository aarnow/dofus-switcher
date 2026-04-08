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
        class="w-9 h-9 rounded-full flex items-center justify-center
             text-[11px] font-bold text-[#5DCAA5] bg-[#1a4560] border border-[#1a4560] flex-shrink-0"
    >
      {{ initials }}
    </div>

    <div class="flex-1 min-w-0">
      <p class="text-[13px] font-bold text-[#c8e8f0] truncate">{{ name }}</p>
      <p class="text-[11px] text-[#4a7a8a]">Client Dofus</p>
    </div>

    <span v-if="active" class="text-[10px] font-bold text-[#1eb8cc] bg-[#0f2a36]
                                border border-[#1eb8cc33] rounded-full px-2 py-0.5">
      ACTIF
    </span>

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
const props = defineProps<{
  name: string
  index: number
  active: boolean
  enabled: boolean
}>()

defineEmits<{ toggle: [] }>()

const initials = computed(() =>
    props.name.slice(0, 2).toUpperCase()
)
</script>