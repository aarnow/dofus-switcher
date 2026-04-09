<template>
  <button
      @click="$emit('click')"
      class="flex items-center justify-center gap-1.5 rounded-lg text-[11px] font-bold
             tracking-wider transition-all duration-150"
      :class="variant === 'danger'
        ? 'px-3 py-2'
        : 'flex-1 py-2'"
      :style="variant === 'danger'
        ? `background: rgba(255,80,80,0.1); border: 1px solid rgba(255,80,80,0.2); color: #ff6b6b;`
        : `background: rgba(93,202,165,0.15); border: 1px solid rgba(93,202,165,0.3); color: #5DCAA5;`"
      @mouseenter="onEnter"
      @mouseleave="onLeave"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
const props = defineProps<{ variant?: 'default' | 'danger' }>()
defineEmits<{ click: [] }>()

function onEnter(e: MouseEvent) {
  const el = e.currentTarget as HTMLElement
  el.style.background = props.variant === 'danger'
      ? 'rgba(255,80,80,0.2)'
      : 'rgba(93,202,165,0.25)'
}

function onLeave(e: MouseEvent) {
  const el = e.currentTarget as HTMLElement
  el.style.background = props.variant === 'danger'
      ? 'rgba(255,80,80,0.1)'
      : 'rgba(93,202,165,0.15)'
}
</script>