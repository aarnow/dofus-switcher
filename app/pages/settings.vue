<template>
  <div class="flex-1 flex flex-col px-6 py-6 gap-4 overflow-y-auto" style="max-height: calc(100vh - 52px);">

    <!-- Overlay -->
    <div class="flex items-center justify-between px-4 py-3 rounded-xl transition-all duration-150"
         style="background: rgba(0,0,0,0.2); border: 1px solid rgba(255,255,255,0.06);">
      <div>
        <p class="text-[13px] font-bold text-[#c8ead8]">Overlay de compte</p>
        <p class="text-[11px] text-[#3a7a5a] mt-0.5">
          Affiche les comptes actifs en haut de l'écran.
        </p>
      </div>
      <button
          @click="toggleOverlay"
          class="w-10 h-5 rounded-full transition-colors duration-200 relative flex-shrink-0"
          :class="overlayEnabled ? 'bg-[#5DCAA5]' : 'bg-[#1a3a2e]'"
      >
        <span
            class="absolute top-0.5 w-4 h-4 bg-white rounded-full transition-all duration-200"
            :class="overlayEnabled ? 'right-0.5' : 'left-0.5'"
        />
      </button>
    </div>

    <!-- Touches de switch -->
    <div class="flex flex-col gap-3 px-4 py-3 rounded-xl"
         style="background: rgba(0,0,0,0.2); border: 1px solid rgba(255,255,255,0.06);">
      <div>
        <p class="text-[13px] font-bold text-[#c8ead8]">Touches de switch</p>
        <p class="text-[11px] text-[#3a7a5a] mt-0.5">
          Appuyez sur "Modifier" puis sur la touche ou le bouton souris souhaité.
        </p>
      </div>

      <div class="flex flex-col gap-2">
        <div v-for="binding in bindings" :key="binding.id"
             class="flex items-center gap-3">
          <span class="text-[11px] font-bold text-[#3a7a5a] uppercase tracking-wider w-24">
            {{ binding.label }}
          </span>
          <span class="font-mono text-[12px] font-bold text-[#5DCAA5] flex-1 text-center
                       rounded-lg px-2 py-1"
                style="background: rgba(93,202,165,0.1); border: 1px solid rgba(93,202,165,0.2);">
            {{ binding.display }}
          </span>
          <button
              @click="startCapture(binding.id)"
              class="text-[11px] transition-colors uppercase tracking-wider"
              :class="capturing === binding.id ? 'text-[#5DCAA5]' : 'text-[#2a6a4e] hover:text-[#5DCAA5]'"
          >
            {{ capturing === binding.id ? 'En attente...' : 'Modifier' }}
          </button>
        </div>
      </div>

      <div v-if="capturing"
           class="rounded-lg p-3 text-center"
           style="background: rgba(93,202,165,0.08); border: 1px solid rgba(93,202,165,0.2);">
        <p class="text-[#c8ead8] text-[11px]">Appuyez sur une touche ou un bouton souris...</p>
        <button @click="cancelCapture"
                class="mt-1 text-[10px] text-[#3a7a5a] hover:text-[#5DCAA5] transition-colors">
          Annuler
        </button>
      </div>
    </div>

    <!-- Informations -->
    <div class="flex flex-col gap-2 px-4 py-3 rounded-xl"
         style="background: rgba(0,0,0,0.2); border: 1px solid rgba(255,255,255,0.06);">
      <p class="text-[13px] font-bold text-[#c8ead8]">Informations</p>

      <div class="flex flex-col gap-1.5 mt-1">
        <div class="flex items-center gap-2">
          <span class="text-[10px] text-[#3a7a5a] uppercase tracking-wider w-28">Application</span>
          <span class="text-[11px] text-[#c8ead8] font-mono flex-1 truncate">{{ appDataPath }}</span>
          <button
              @click="openFolder('app')"
              class="flex items-center justify-center rounded transition-all duration-150 flex-shrink-0"
              style="width: 22px; height: 22px; background: rgba(93,202,165,0.1); border: 1px solid rgba(93,202,165,0.2);"
              @mouseenter="e => (e.currentTarget as HTMLElement).style.background='rgba(93,202,165,0.25)'"
              @mouseleave="e => (e.currentTarget as HTMLElement).style.background='rgba(93,202,165,0.1)'"
              title="Ouvrir le dossier"
          >
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="#5DCAA5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
          </button>
        </div>
        <div class="flex items-center gap-2">
          <span class="text-[10px] text-[#3a7a5a] uppercase tracking-wider w-28">Préférences</span>
          <span class="text-[11px] text-[#c8ead8] font-mono flex-1 truncate">{{ configPath }}</span>
          <button
              @click="openFolder('config')"
              class="flex items-center justify-center rounded transition-all duration-150 flex-shrink-0"
              style="width: 22px; height: 22px; background: rgba(93,202,165,0.1); border: 1px solid rgba(93,202,165,0.2);"
              @mouseenter="e => (e.currentTarget as HTMLElement).style.background='rgba(93,202,165,0.25)'"
              @mouseleave="e => (e.currentTarget as HTMLElement).style.background='rgba(93,202,165,0.1)'"
              title="Ouvrir le dossier"
          >
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="#5DCAA5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
          </button>
        </div>
        <div class="flex items-center gap-2">
          <span class="text-[10px] text-[#3a7a5a] uppercase tracking-wider w-28">Version</span>
          <span class="text-[11px] text-[#c8ead8] font-mono">v0.1.0</span>
        </div>
      </div>
    </div>

    <!-- Zone danger -->
    <div class="flex flex-col gap-3 px-4 py-3 rounded-xl mt-auto"
         style="background: rgba(0,0,0,0.2); border: 1px solid rgba(255,60,60,0.45);">
      <div>
        <p class="text-[13px] font-bold text-[#ff8080]">Zone de danger</p>
        <p class="text-[11px] text-[#8a5a5a] mt-0.5">
          Ces actions sont irréversibles.
        </p>
      </div>

      <div v-if="!confirmUninstall" class="flex gap-2">
        <button
            @click="confirmUninstall = true"
            class="flex items-center gap-1.5 px-4 py-2 rounded-lg text-[11px] font-bold
                   transition-all duration-150"
            style="background: rgba(255,60,60,0.1); border: 1px solid rgba(255,60,60,0.25); color: #ff6b6b;"
            @mouseenter="e => (e.currentTarget as HTMLElement).style.background='rgba(255,60,60,0.2)'"
            @mouseleave="e => (e.currentTarget as HTMLElement).style.background='rgba(255,60,60,0.1)'"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/><path d="M9 6V4h6v2"/></svg>
          Désinstaller l'application
        </button>
      </div>

      <div v-else class="flex items-center gap-3">
        <span class="text-[11px] text-[#ff8080] font-bold flex-1">
          Supprimer l'application et toutes les données ?
        </span>
        <button @click="uninstall"
                class="text-[10px] font-bold px-3 py-1.5 rounded-lg transition-colors"
                style="background: rgba(255,60,60,0.2); border: 1px solid rgba(255,60,60,0.4); color: #ff6b6b;">
          Confirmer
        </button>
        <button @click="confirmUninstall = false"
                class="text-[10px] font-bold px-3 py-1.5 rounded-lg transition-colors"
                style="background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.08); color: #3a7a5a;">
          Annuler
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
const confirmUninstall = ref(false)

const appDataPath = ref('')
const configPath = ref('')

const bindings = ref([
  { id: 'next', label: 'Suivant',   display: 'Mouse5', value: { type: 'mouse', code: 2 } },
  { id: 'prev', label: 'Précédent', display: 'Mouse4', value: { type: 'mouse', code: 1 } },
])

async function getStore() {
  return await load('settings.json')
}

async function openFolder(type: 'app' | 'config') {
  await invoke('open_folder', { folderType: type })
}

onMounted(async () => {
  const store = await getStore()

  const savedOverlay = await store.get<boolean>('overlayEnabled')
  if (savedOverlay !== null && savedOverlay !== undefined) {
    overlayEnabled.value = savedOverlay
  }

  const savedBindings = await store.get<typeof bindings.value>('bindings')
  if (savedBindings && savedBindings.length > 0) {
    bindings.value = savedBindings
  }

  // Chemins
  const appData = await invoke<string>('get_app_paths')
  const parts = appData.split('|')
  appDataPath.value = parts[0] ?? ''
  configPath.value = parts[1] ?? ''

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
  await saveHotkeys()
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

async function uninstall() {
  await invoke('uninstall_app')
}
</script>