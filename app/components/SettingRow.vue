<template>
  <div class="flex items-center justify-between">
    <div>
      <p class="text-[13px] font-bold text-[#c8ead8]">{{ title }}</p>
      <p v-if="description" class="text-[11px] text-[#3a7a5a] mt-0.5">{{ description }}</p>
    </div>
    <slot />
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  description?: string
}>()
</script>