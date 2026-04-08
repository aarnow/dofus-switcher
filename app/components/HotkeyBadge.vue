<template>
  <div class="flex items-center gap-3 bg-[#122530] border border-[#1a3a4a]
              rounded-md px-3 py-2 hover:border-[#1eb8cc44] transition-colors">
    <span class="text-[11px] font-bold text-[#4a7a8a] uppercase tracking-wider w-20">
      {{ label }}
    </span>
    <span class="font-mono text-[12px] font-bold text-[#1eb8cc] bg-[#0f2a36]
                 border border-[#1eb8cc33] rounded px-2 py-0.5">
      {{ keyVal }}
    </span>
    <button
        @click="$emit('edit')"
        class="ml-auto text-[10px] text-[#2a6070] hover:text-[#1eb8cc] transition-colors uppercase tracking-wider"
    >
      Modifier
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{ label: string; keyVal: string }>()
defineEmits<{ edit: [] }>()
</script>