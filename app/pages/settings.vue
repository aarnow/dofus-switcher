<template>
  <div class="flex-1 flex flex-col px-6 py-6 gap-4 overflow-y-auto" style="max-height: calc(100vh - 52px);">

    <!-- Overlay -->
    <SettingCard>
      <SettingRow title="Overlay de compte" description="Affiche les comptes actifs en haut de l'écran.">
        <AppToggle v-model="overlayEnabled" @update:modelValue="toggleOverlay" />
      </SettingRow>
    </SettingCard>

    <!-- Variant overlay -->
    <SettingCard>
      <p class="text-[13px] font-bold text-[#c8ead8]">Apparence de l'overlay</p>
      <p class="text-[11px] text-[#3a7a5a] mt-0.5">Choisissez le style de l'overlay.</p>

      <div class="flex gap-2 mt-2">
        <button
            @click="saveOverlayVariant('default')"
            class="flex-1 py-2 rounded-lg text-[11px] font-bold transition-all duration-150"
            :style="overlayVariant === 'default'
          ? 'background: rgba(93,202,165,0.2); border: 1px solid rgba(93,202,165,0.5); color: #5DCAA5;'
          : 'background: rgba(0,0,0,0.2); border: 1px solid rgba(255,255,255,0.08); color: #3a7a5a;'"
        >
          Par défaut
        </button>
        <button
            @click="saveOverlayVariant('compact')"
            class="flex-1 py-2 rounded-lg text-[11px] font-bold transition-all duration-150"
            :style="overlayVariant === 'compact'
          ? 'background: rgba(93,202,165,0.2); border: 1px solid rgba(93,202,165,0.5); color: #5DCAA5;'
          : 'background: rgba(0,0,0,0.2); border: 1px solid rgba(255,255,255,0.08); color: #3a7a5a;'"
        >
          Compact
        </button>
      </div>

      <div v-if="overlayVariant === 'compact'" class="flex flex-col gap-3 mt-3">

        <!-- Dimensions -->
        <div class="flex items-center gap-3">
          <span class="text-[11px] text-[#3a7a5a] w-20">Largeur</span>
          <input
              v-model.number="compactWidth"
              @change="saveCompactDimensions"
              type="number" min="200" max="1920" step="1"
              class="flex-1 rounded-lg px-3 py-1.5 text-[12px] font-bold text-[#c8ead8] outline-none"
              style="background: rgba(0,0,0,0.25); border: 1px solid rgba(255,255,255,0.08);"
              @focus="e => (e.target as HTMLElement).style.borderColor='rgba(93,202,165,0.4)'"
              @blur="e => (e.target as HTMLElement).style.borderColor='rgba(255,255,255,0.08)'"
          />
          <span class="text-[11px] text-[#3a7a5a]">px</span>
        </div>

        <div class="flex items-center gap-3">
          <span class="text-[11px] text-[#3a7a5a] w-20">Hauteur</span>
          <input
              v-model.number="compactHeight"
              @change="saveCompactDimensions"
              type="number" min="40" max="500" step="1"
              class="flex-1 rounded-lg px-3 py-1.5 text-[12px] font-bold text-[#c8ead8] outline-none"
              style="background: rgba(0,0,0,0.25); border: 1px solid rgba(255,255,255,0.08);"
              @focus="e => (e.target as HTMLElement).style.borderColor='rgba(93,202,165,0.4)'"
              @blur="e => (e.target as HTMLElement).style.borderColor='rgba(255,255,255,0.08)'"
          />
          <span class="text-[11px] text-[#3a7a5a]">px</span>
        </div>

        <!-- Séparateur -->
        <div class="w-full h-px mt-1" style="background: rgba(255,255,255,0.05);"/>
        <p class="text-[11px] font-bold text-[#c8ead8]">Position</p>

        <div class="flex items-center gap-3">
          <span class="text-[11px] text-[#3a7a5a] w-20">X</span>
          <input
              v-model.number="compactX"
              @change="saveCompactPosition"
              type="number" min="0" max="3840" step="1"
              class="flex-1 rounded-lg px-3 py-1.5 text-[12px] font-bold text-[#c8ead8] outline-none"
              style="background: rgba(0,0,0,0.25); border: 1px solid rgba(255,255,255,0.08);"
              @focus="e => (e.target as HTMLElement).style.borderColor='rgba(93,202,165,0.4)'"
              @blur="e => (e.target as HTMLElement).style.borderColor='rgba(255,255,255,0.08)'"
          />
          <span class="text-[11px] text-[#3a7a5a]">px</span>
        </div>

        <div class="flex items-center gap-3">
          <span class="text-[11px] text-[#3a7a5a] w-20">Y</span>
          <input
              v-model.number="compactY"
              @change="saveCompactPosition"
              type="number" min="0" max="2160" step="1"
              class="flex-1 rounded-lg px-3 py-1.5 text-[12px] font-bold text-[#c8ead8] outline-none"
              style="background: rgba(0,0,0,0.25); border: 1px solid rgba(255,255,255,0.08);"
              @focus="e => (e.target as HTMLElement).style.borderColor='rgba(93,202,165,0.4)'"
              @blur="e => (e.target as HTMLElement).style.borderColor='rgba(255,255,255,0.08)'"
          />
          <span class="text-[11px] text-[#3a7a5a]">px</span>
        </div>

        <button
            @click="resetCompactPosition"
            class="self-start flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-[11px] font-bold transition-all duration-150"
            style="background: rgba(0,0,0,0.2); border: 1px solid rgba(255,255,255,0.08); color: #3a7a5a;"
            @mouseenter="e => (e.currentTarget as HTMLElement).style.borderColor='rgba(93,202,165,0.3)'"
            @mouseleave="e => (e.currentTarget as HTMLElement).style.borderColor='rgba(255,255,255,0.08)'"
        >
          <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
            <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/>
            <path d="M3 3v5h5"/>
          </svg>
          Réinitialiser la position
        </button>
      </div>

    </SettingCard>

    <!-- Touches -->
    <SettingCard>
      <SettingRow title="Touches de switch" description="Appuyez sur &quot;Modifier&quot; puis sur la touche ou le bouton souris souhaité." />

      <HotkeyBinding
          v-for="binding in bindings"
          :key="binding.id"
          :label="binding.label"
          :display="binding.display"
          :capturing="capturing === binding.id"
          @capture="startCapture(binding.id)"
      />

      <div v-if="capturing"
           class="rounded-lg p-3 text-center"
           style="background: rgba(93,202,165,0.08); border: 1px solid rgba(93,202,165,0.2);">
        <p class="text-[#c8ead8] text-[11px]">Appuyez sur une touche ou un bouton souris...</p>
        <button @click="cancelCapture"
                class="mt-1 text-[10px] text-[#3a7a5a] hover:text-[#5DCAA5] transition-colors">
          Annuler
        </button>
      </div>
    </SettingCard>

    <!-- Informations -->
    <SettingCard>
      <p class="text-[13px] font-bold text-[#c8ead8]">Informations</p>
      <div class="flex flex-col gap-1.5 mt-1">
        <div v-for="info in infos" :key="info.label" class="flex items-center gap-2">
          <span class="text-[10px] text-[#3a7a5a] uppercase tracking-wider w-28">{{ info.label }}</span>
          <span class="text-[11px] text-[#c8ead8] font-mono flex-1 truncate">{{ info.value }}</span>
          <button
              v-if="info.folder"
              @click="openFolder(info.folder)"
              class="flex items-center justify-center rounded transition-all duration-150 flex-shrink-0"
              style="width: 22px; height: 22px; background: rgba(93,202,165,0.1); border: 1px solid rgba(93,202,165,0.2);"
              @mouseenter="e => (e.currentTarget as HTMLElement).style.background='rgba(93,202,165,0.25)'"
              @mouseleave="e => (e.currentTarget as HTMLElement).style.background='rgba(93,202,165,0.1)'"
          >
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="#5DCAA5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
          </button>
        </div>
      </div>
    </SettingCard>

    <!-- Zone danger -->
    <div class="flex flex-col gap-3 px-4 py-3 rounded-xl mt-auto"
         style="background: rgba(0,0,0,0.2); border: 1px solid rgba(255,60,60,0.45);">
      <div>
        <p class="text-[13px] font-bold text-[#ff8080]">Zone de danger</p>
        <p class="text-[11px] text-[#8a5a5a] mt-0.5">Ces actions sont irréversibles.</p>
      </div>

      <div v-if="!confirmUninstall">
        <button
            @click="confirmUninstall = true"
            class="flex items-center gap-1.5 px-4 py-2 rounded-lg text-[11px] font-bold transition-all duration-150"
            style="background: rgba(255,60,60,0.1); border: 1px solid rgba(255,60,60,0.25); color: #ff6b6b;"
            @mouseenter="e => (e.currentTarget as HTMLElement).style.background='rgba(255,60,60,0.2)'"
            @mouseleave="e => (e.currentTarget as HTMLElement).style.background='rgba(255,60,60,0.1)'"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
            <polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/><path d="M9 6V4h6v2"/>
          </svg>
          Désinstaller l'application
        </button>
      </div>

      <div v-else class="flex items-center gap-3">
        <span class="text-[11px] text-[#ff8080] font-bold flex-1">Supprimer l'application et toutes les données ?</span>
        <button @click="uninstall"
                class="text-[10px] font-bold px-3 py-1.5 rounded-lg"
                style="background: rgba(255,60,60,0.2); border: 1px solid rgba(255,60,60,0.4); color: #ff6b6b;">
          Confirmer
        </button>
        <button @click="confirmUninstall = false"
                class="text-[10px] font-bold px-3 py-1.5 rounded-lg"
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

const overlayVariant = ref<'default' | 'compact'>('default')
const compactWidth = ref(600)
const compactHeight = ref(80)
const compactX = ref(-1) // -1 = centré par défaut
const compactY = ref(12)

const bindings = ref([
  { id: 'next', label: 'Suivant',   display: 'Mouse5', value: { type: 'mouse', code: 2 } },
  { id: 'prev', label: 'Précédent', display: 'Mouse4', value: { type: 'mouse', code: 1 } },
])

const infos = ref([
  { label: 'Application', value: '', folder: 'app' as const },
  { label: 'Préférences', value: '', folder: 'config' as const },
  { label: 'Version',     value: 'v0.1.0', folder: null },
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

  const savedVariant = await store.get<string>('overlayVariant')
  if (savedVariant) overlayVariant.value = savedVariant as 'default' | 'compact'

  const savedCompactWidth = await store.get<number>('compactWidth')
  if (savedCompactWidth) compactWidth.value = savedCompactWidth

  const savedCompactHeight = await store.get<number>('compactHeight')
  if (savedCompactHeight) compactHeight.value = savedCompactHeight

  const savedX = await store.get<number>('compactX')
  const savedY = await store.get<number>('compactY')
  if (savedX !== null && savedX !== undefined) compactX.value = savedX
  if (savedY !== null && savedY !== undefined) compactY.value = savedY

  const appData = await invoke<string>('get_app_paths')
  const parts = appData.split('|')
  if(infos.value[0]) infos.value[0].value = parts[0] ?? ''
  if(infos.value[1]) infos.value[1].value = parts[1] ?? ''

  window.addEventListener('keydown', (e) => {
    if (!capturing.value) return
    e.preventDefault()
    stopCapture(e.key.toUpperCase(), 'key', +e.code)
  })

  window.addEventListener('mousedown', (e) => {
    if (!capturing.value) return
    if (e.button === 3) stopCapture('Mouse4', 'mouse', 1)
    else if (e.button === 4) stopCapture('Mouse5', 'mouse', 2)
  })
})

async function saveOverlayVariant(variant: 'default' | 'compact') {
  overlayVariant.value = variant
  const store = await getStore()
  await store.set('overlayVariant', variant)
  await store.save()
}

async function saveCompactDimensions() {
  const store = await getStore()
  await store.set('compactWidth', compactWidth.value)
  await store.set('compactHeight', compactHeight.value)
  await store.save()
  await invoke('resize_compact_overlay', {
    width: compactWidth.value,
    height: compactHeight.value,
  })
}

async function saveCompactPosition() {
  const store = await getStore()
  await store.set('compactX', compactX.value)
  await store.set('compactY', compactY.value)
  await store.save()
  await invoke('move_compact_overlay', { x: compactX.value, y: compactY.value })
}

async function resetCompactPosition() {
  compactX.value = -1
  compactY.value = 12
  const store = await getStore()
  await store.set('compactX', -1)
  await store.set('compactY', 12)
  await store.save()
  await invoke('move_compact_overlay', { x: -1, y: 12 })
}

async function toggleOverlay() {
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