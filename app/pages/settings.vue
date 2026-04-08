<template>
  <div class="flex-1 flex flex-col px-6 py-6 gap-6">

    <div>
      <h1 class="text-[15px] font-bold text-[#e0f0f5]">Options</h1>
      <p class="text-[11px] text-[#4a7a8a] mt-1">Configuration générale de l'application.</p>
    </div>

    <div class="flex flex-col gap-4">

      <!-- Overlay -->
      <div class="flex items-center justify-between bg-[#122530] border border-[#1a3a4a]
                  rounded-md px-4 py-3 hover:border-[#1eb8cc44] transition-colors">
        <div>
          <p class="text-[13px] font-bold text-[#c8e8f0]">Overlay de compte</p>
          <p class="text-[11px] text-[#4a7a8a] mt-0.5">
            Affiche le numéro du compte actif en haut de l'écran.
          </p>
        </div>
        <button
            @click="toggleOverlay"
            class="w-10 h-5 rounded-full transition-colors duration-200 relative flex-shrink-0"
            :class="overlayEnabled ? 'bg-[#1eb8cc]' : 'bg-[#1a3a4a]'"
        >
          <span
              class="absolute top-0.5 w-4 h-4 bg-white rounded-full transition-all duration-200"
              :class="overlayEnabled ? 'right-0.5' : 'left-0.5'"
          />
        </button>
      </div>

      <!-- Touches -->
      <div class="bg-[#122530] border border-[#1a3a4a] rounded-md px-4 py-3">
        <p class="text-[13px] font-bold text-[#c8e8f0] mb-1">Touches de switch</p>
        <p class="text-[11px] text-[#4a7a8a] mb-3">
          Appuyez sur "Modifier" puis sur la touche ou le bouton souris souhaité.
        </p>

        <div class="flex flex-col gap-2">
          <div v-for="binding in bindings" :key="binding.id"
               class="flex items-center gap-3">
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
             class="mt-3 bg-[#0f2a36] border border-[#1eb8cc44] rounded p-3 text-center">
          <p class="text-[#e0f0f5] text-[11px]">Appuyez sur une touche ou un bouton souris...</p>
          <button @click="cancelCapture"
                  class="mt-1 text-[10px] text-[#4a7a8a] hover:text-[#1eb8cc] transition-colors">
            Annuler
          </button>
        </div>

        <button
            @click="saveHotkeys"
            class="mt-4 bg-[#1eb8cc] text-[#091820] font-bold text-[11px] tracking-widest
                 rounded-full px-6 py-1.5 hover:bg-[#28d4e8] transition-colors"
        >
          Enregistrer
        </button>
      </div>

    </div>

  </div>
</template>

<script setup lang="ts">
import { isCapturing } from '~/composables/useCapturing'
import { invoke } from '@tauri-apps/api/core'
import { load } from '@tauri-apps/plugin-store'

const overlayEnabled = ref(true)
const capturing = ref<string | null>(null)

const bindings = ref([
  { id: 'next', label: 'Suivant',   display: 'Mouse5', value: { type: 'mouse', code: 2 } },
  { id: 'prev', label: 'Précédent', display: 'Mouse4', value: { type: 'mouse', code: 1 } },
])

async function getStore() {
  return await load('settings.json')
}

onMounted(async () => {
  const store = await getStore()

  // Charge uniquement pour afficher les valeurs sauvegardées dans l'UI
  const savedOverlay = await store.get<boolean>('overlayEnabled')
  if (savedOverlay !== null && savedOverlay !== undefined) {
    overlayEnabled.value = savedOverlay
  }

  const savedBindings = await store.get<typeof bindings.value>('bindings')
  if (savedBindings && savedBindings.length > 0) {
    bindings.value = savedBindings
  }

  // Listeners capture
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

async function toggleOverlay() {
  overlayEnabled.value = !overlayEnabled.value
  await invoke('toggle_overlay', { enabled: overlayEnabled.value })
  const store = await getStore()
  await store.set('overlayEnabled', overlayEnabled.value)
  await store.save()
}

async function startCapture(id: string) {
  capturing.value = id
  isCapturing.value = true
  await invoke('set_hook_enabled', { enabled: false })
}

async function stopCapture(display: string, type: 'key' | 'mouse', code: number) {
  const binding = bindings.value.find(b => b.id === capturing.value)
  if (binding) {
    binding.display = display
    binding.value = { type, code }
  }
  capturing.value = null
  isCapturing.value = false
  await invoke('set_hook_enabled', { enabled: true })
}

async function cancelCapture() {
  capturing.value = null
  isCapturing.value = false
  await invoke('set_hook_enabled', { enabled: true })
}

async function saveHotkeys() {
  const next = bindings.value.find(b => b.id === 'next')!.value
  const prev = bindings.value.find(b => b.id === 'prev')!.value
  await invoke('update_hotkeys', {
    keyNext:   next.type === 'key'   ? next.code : 0,
    keyPrev:   prev.type === 'key'   ? prev.code : 0,
    mouseNext: next.type === 'mouse' ? next.code : 0,
    mousePrev: prev.type === 'mouse' ? prev.code : 0,
  })
  const store = await getStore()
  await store.set('bindings', bindings.value)
  await store.save()
}
</script>