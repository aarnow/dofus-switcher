<template>
  <div>
    <label class="text-[10px] font-bold text-[#3a7a5a] uppercase tracking-wider flex items-center gap-1.5">
      <slot name="label-icon" />
      {{ label }}
    </label>
    <slot />
  </div>
</template>

<script setup lang="ts">
defineProps<{ label: string }>()
</script>