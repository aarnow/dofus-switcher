<template>
  <div class="flex-1 flex overflow-hidden">

    <!-- Colonne gauche : liste -->
    <div class="flex flex-col border-r flex-shrink-0"
         style="width: 260px; border-color: rgba(255,255,255,0.06);">

      <div class="flex-1 overflow-y-auto px-3 py-3 flex flex-col gap-1.5">
        <div v-if="characters.length === 0"
             class="flex-1 flex items-center justify-center text-[#1a4a3a] text-sm text-center px-4">
          Aucun personnage.<br>Cliquez sur + pour en créer un.
        </div>
        <CharacterRow
            v-for="char in characters"
            :key="char.id"
            :char="char"
            :selected="selectedId === char.id"
            @click="selectCharacter(char)"
        />
      </div>

      <div class="flex gap-2 px-3 py-3 border-t flex-shrink-0"
           style="border-color: rgba(255,255,255,0.06);">
        <ActionButton @click="createNew">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
            <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
          </svg>
          Nouveau
        </ActionButton>
        <ActionButton v-if="selectedId" variant="danger" @click="confirmDelete = true">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
            <polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/>
            <path d="M10 11v6"/><path d="M14 11v6"/><path d="M9 6V4h6v2"/>
          </svg>
          Supprimer
        </ActionButton>
      </div>

      <div v-if="confirmDelete" class="px-3 pb-3 flex items-center gap-2">
        <span class="text-[10px] text-[#ff6b6b] font-bold flex-1">Supprimer ce personnage ?</span>
        <button @click="deleteSelected"
                class="text-[9px] font-bold px-2 py-1 rounded"
                style="background: rgba(255,80,80,0.2); border: 1px solid rgba(255,80,80,0.4); color: #ff6b6b;">
          Oui
        </button>
        <button @click="confirmDelete = false"
                class="text-[9px] font-bold px-2 py-1 rounded"
                style="background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.1); color: #3a7a5a;">
          Non
        </button>
      </div>
    </div>

    <!-- Colonne droite : édition -->
    <div class="flex-1 flex flex-col overflow-y-auto">

      <div v-if="!form" class="flex-1 flex items-center justify-center">
        <p class="text-[#1a4a3a] text-sm">Sélectionnez ou créez un personnage</p>
      </div>

      <div v-else class="flex flex-col gap-5 px-6 py-6">

        <!-- Avatar + pseudo -->
        <div class="flex items-center gap-4">
          <div class="w-16 h-16 rounded-full overflow-hidden border-2 flex-shrink-0"
               :style="{ borderColor: classColor(form.classe) || '#1a4a3a' }">
            <img v-if="form.classe && classImage(form.classe)"
                 :src="classImage(form.classe)" :alt="form.classe"
                 class="w-full h-full object-cover" />
            <div v-else class="w-full h-full flex items-center justify-center text-xl font-bold"
                 :style="{ color: classColor(form.classe) || '#3a7a5a', background: 'rgba(0,0,0,0.3)' }">
              {{ form.pseudo ? form.pseudo.slice(0, 2).toUpperCase() : '?' }}
            </div>
          </div>
          <div class="flex-1">
            <FormField label="Pseudo">
              <input
                  v-model="form.pseudo" @input="autoSave"
                  placeholder="Nom du personnage"
                  class="mt-1 w-full rounded-lg px-3 py-2 text-[13px] font-bold text-[#c8ead8] outline-none transition-colors"
                  style="background: rgba(0,0,0,0.25); border: 1px solid rgba(255,255,255,0.08);"
                  @focus="e => (e.target as HTMLElement).style.borderColor='rgba(93,202,165,0.4)'"
                  @blur="e => (e.target as HTMLElement).style.borderColor='rgba(255,255,255,0.08)'"
              />
            </FormField>
          </div>
        </div>

        <FormDivider />

        <FormField label="Classe">
          <select
              v-model="form.classe" @change="autoSave"
              class="mt-1 w-full rounded-lg px-3 py-2 text-[13px] text-[#c8ead8] outline-none transition-colors"
              style="background: rgba(0,0,0,0.25); border: 1px solid rgba(255,255,255,0.08);"
          >
            <option value="">Sélectionner...</option>
            <option v-for="c in classes" :key="c" :value="c">{{ c }}</option>
          </select>
        </FormField>

        <FormField label="Initiative">
          <template #label-icon>
            <img src="/icons/initiative.png" alt="initiative" class="w-3.5 h-3.5 object-contain opacity-70" />
          </template>
          <input
              v-model.number="form.initiative" @input="autoSave"
              type="number" min="0" placeholder="0"
              class="mt-1 w-full rounded-lg px-3 py-2 text-[13px] text-[#C268D7] font-bold outline-none transition-colors"
              style="background: rgba(194,104,215,0.08); border: 1px solid rgba(194,104,215,0.2);"
              @focus="e => (e.target as HTMLElement).style.borderColor='rgba(194,104,215,0.5)'"
              @blur="e => (e.target as HTMLElement).style.borderColor='rgba(194,104,215,0.2)'"
          />
        </FormField>

        <FormDivider />

        <FormField label="Éléments">
          <div class="flex flex-wrap gap-2 mt-2">
            <button
                v-for="(el, name) in ELEMENTS" :key="name"
                @click="toggleSelection(form.elements, name); autoSave()"
                class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-[11px] font-bold border transition-all"
                :style="form.elements.includes(name)
                  ? { background: el.color + '22', borderColor: el.color, color: el.color }
                  : { background: 'rgba(0,0,0,0.2)', borderColor: 'rgba(255,255,255,0.08)', color: '#3a7a5a' }"
            >
              <img :src="el.image" :alt="name" class="w-4 h-4 object-contain" />
              {{ name }}
            </button>
          </div>
        </FormField>

        <FormDivider />

        <FormField label="Rôles">
          <div class="flex flex-wrap gap-2 mt-2">
            <button
                v-for="(role, name) in ROLES" :key="name"
                @click="toggleSelection(form.roles, name); autoSave()"
                class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-[11px] font-bold border transition-all"
                :style="form.roles.includes(name)
                  ? { background: 'rgba(93,202,165,0.15)', borderColor: 'rgba(93,202,165,0.4)', color: '#5DCAA5' }
                  : { background: 'rgba(0,0,0,0.2)', borderColor: 'rgba(255,255,255,0.08)', color: '#3a7a5a' }"
            >
              <img :src="role.image" :alt="name" class="w-4 h-4 object-contain" />
              {{ name }}
            </button>
          </div>
        </FormField>

      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { load } from '@tauri-apps/plugin-store'
import { CLASSES, ELEMENTS, ROLES, classColor, classImage } from '~/composables/useCharacters'
import type { Character } from '~/composables/useCharacters'

const classes = Object.keys(CLASSES).sort()
const characters = ref<Character[]>([])
const selectedId = ref<string | null>(null)
const confirmDelete = ref(false)
const form = ref<Character | null>(null)
let saveTimer: ReturnType<typeof setTimeout> | null = null

async function getStore() {
  return await load('settings.json')
}

onMounted(async () => {
  const store = await getStore()
  const saved = await store.get<Character[]>('characters')
  if (saved && saved.length > 0) {
    characters.value = saved
    if (characters.value[0]) selectCharacter(characters.value[0])
  }
})

function selectCharacter(char: Character) {
  selectedId.value = char.id
  form.value = { ...char, elements: [...char.elements], roles: [...char.roles] }
  confirmDelete.value = false
}

function createNew() {
  const newChar: Character = {
    id: crypto.randomUUID(),
    pseudo: '',
    classe: '',
    elements: [],
    roles: [],
    initiative: 0,
  }
  characters.value.push(newChar)
  selectCharacter(newChar)
}

function toggleSelection(arr: string[], item: string) {
  const idx = arr.indexOf(item)
  if (idx === -1) arr.push(item)
  else arr.splice(idx, 1)
}

function autoSave() {
  if (!form.value) return
  const idx = characters.value.findIndex(c => c.id === form.value!.id)
  if (idx !== -1) characters.value[idx] = { ...form.value }
  if (saveTimer) clearTimeout(saveTimer)
  saveTimer = setTimeout(async () => {
    const store = await getStore()
    await store.set('characters', characters.value)
    await store.save()
  }, 500)
}

async function deleteSelected() {
  if (!selectedId.value) return
  characters.value = characters.value.filter(c => c.id !== selectedId.value)
  const store = await getStore()
  await store.set('characters', characters.value)
  await store.save()
  selectedId.value = null
  form.value = null
  confirmDelete.value = false
  if (characters.value[0]) selectCharacter(characters.value[0])
}
</script>