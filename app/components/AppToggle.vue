<template>
  <button
      @click="$emit('update:modelValue', !modelValue)"
      class="w-10 h-5 rounded-full transition-colors duration-200 relative flex-shrink-0"
      :class="modelValue ? 'bg-[#5DCAA5]' : 'bg-[#1a3a2e]'"
  >
    <span
        class="absolute top-0.5 w-4 h-4 bg-white rounded-full transition-all duration-200"
        :class="modelValue ? 'right-0.5' : 'left-0.5'"
    />
  </button>
</template>

<script setup lang="ts">
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [value: boolean] }>()
</script>