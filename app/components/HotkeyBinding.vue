<template>
  <div class="flex items-center gap-3">
    <span class="text-[11px] font-bold text-[#3a7a5a] uppercase tracking-wider w-24">
      {{ label }}
    </span>
    <span class="font-mono text-[12px] font-bold text-[#5DCAA5] flex-1 text-center rounded-lg px-2 py-1"
          style="background: rgba(93,202,165,0.1); border: 1px solid rgba(93,202,165,0.2);">
      {{ display }}
    </span>
    <button
        @click="$emit('capture')"
        class="text-[11px] transition-colors uppercase tracking-wider"
        :class="capturing ? 'text-[#5DCAA5]' : 'text-[#2a6a4e] hover:text-[#5DCAA5]'"
    >
      {{ capturing ? 'En attente...' : 'Modifier' }}
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  label: string
  display: string
  capturing: boolean
}>()

defineEmits<{ capture: [] }>()
</script>