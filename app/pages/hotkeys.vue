<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'

const capturing = ref<string | null>(null)

const bindings = ref([
  { id: 'next', label: 'Suivant',   display: 'Mouse5', value: { type: 'mouse', code: 2 } },
  { id: 'prev', label: 'Précédent', display: 'Mouse4', value: { type: 'mouse', code: 1 } },
])

function startCapture(id: string) {
  capturing.value = id
}

function stopCapture(display: string, type: 'key' | 'mouse', code: number) {
  const binding = bindings.value.find(b => b.id === capturing.value)
  if (binding) {
    binding.display = display
    binding.value = { type, code }
  }
  capturing.value = null
}

onMounted(() => {
  window.addEventListener('keydown', (e) => {
    if (!capturing.value) return
    e.preventDefault()
    stopCapture(e.key.toUpperCase(), 'key', e.keyCode)
  })

  window.addEventListener('mousedown', (e) => {
    if (!capturing.value) return
    if (e.button === 3) stopCapture('Mouse4', 'mouse', 1)
    else if (e.button === 4) stopCapture('Mouse5', 'mouse', 2)
  })
})

async function save() {
  const next = bindings.value.find(b => b.id === 'next')!.value
  const prev = bindings.value.find(b => b.id === 'prev')!.value

  await invoke('update_hotkeys', {
    keyNext:   next.type === 'key'   ? next.code : 0,
    keyPrev:   prev.type === 'key'   ? prev.code : 0,
    mouseNext: next.type === 'mouse' ? next.code : 0,
    mousePrev: prev.type === 'mouse' ? prev.code : 0,
  })
}
</script>

<template>
  <div class="flex-1 flex flex-col px-6 py-6 gap-6">

    <div>
      <h1 class="text-[15px] font-bold text-[#e0f0f5]">Configuration des touches</h1>
      <p class="text-[11px] text-[#4a7a8a] mt-1 leading-relaxed">
        Appuyez sur "Modifier" puis sur la touche ou le bouton souris souhaité.
      </p>
    </div>

    <div class="flex flex-col gap-3">
      <div v-for="binding in bindings" :key="binding.id"
           class="flex items-center gap-3 bg-[#122530] border border-[#1a3a4a]
                  rounded-md px-4 py-3 hover:border-[#1eb8cc44] transition-colors">
        <span class="text-[11px] font-bold text-[#4a7a8a] uppercase tracking-wider w-24">
          {{ binding.label }}
        </span>
        <span class="font-mono text-[12px] font-bold text-[#1eb8cc] bg-[#0f2a36]
                     border border-[#1eb8cc33] rounded px-2 py-0.5 flex-1 text-center">
          {{ binding.display }}
        </span>
        <button
            @click="startCapture(binding.id)"
            class="text-[11px] transition-colors uppercase tracking-wider"
            :class="capturing === binding.id ? 'text-[#1eb8cc]' : 'text-[#2a6070] hover:text-[#1eb8cc]'"
        >
          {{ capturing === binding.id ? 'En attente...' : 'Modifier' }}
        </button>
      </div>
    </div>

    <div v-if="capturing"
         class="bg-[#122530] border border-[#1eb8cc44] rounded-lg p-4 text-center">
      <p class="text-[#e0f0f5] text-sm">Appuyez sur une touche ou un bouton souris...</p>
      <button @click="capturing = null"
              class="mt-2 text-[11px] text-[#4a7a8a] hover:text-[#1eb8cc] transition-colors">
        Annuler
      </button>
    </div>

    <div class="mt-auto border-t border-[#0f2a36] pt-4">
      <button
          @click="save"
          class="bg-[#1eb8cc] text-[#091820] font-bold text-[12px] tracking-widest
               rounded-full px-8 py-2 hover:bg-[#28d4e8] transition-colors"
      >
        Enregistrer
      </button>
    </div>

  </div>
</template>