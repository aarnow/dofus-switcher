<template>
  <div class="flex-1 flex overflow-hidden">

    <div class="flex-1 flex flex-col px-6 py-6 gap-4 overflow-y-auto">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-[15px] font-bold text-[#e0f0f5]">Personnages</h1>
          <p class="text-[11px] text-[#4a7a8a] mt-1">
            Créez vos profils pour les associer automatiquement à vos clients.
          </p>
        </div>
        <button
            @click="openForm()"
            class="border border-[#1eb8cc] text-[#1eb8cc] text-[11px] font-bold
                 tracking-wider rounded-full px-4 py-1.5 hover:bg-[#1eb8cc]
                 hover:text-[#091820] transition-all duration-150"
        >
          + Nouveau
        </button>
      </div>

      <div v-if="characters.length === 0"
           class="flex-1 flex items-center justify-center text-[#2a5060] text-sm">
        Aucun personnage créé
      </div>

      <div class="flex flex-col gap-2">
        <div
            v-for="char in characters"
            :key="char.id"
            class="flex items-center gap-3 px-4 py-3 bg-[#122530] border border-[#1a3a4a]
                 rounded-md hover:border-[#2a5a6a] transition-colors cursor-pointer"
            @click="openForm(char)"
        >
          <div
              class="w-9 h-9 rounded-full overflow-hidden flex items-center justify-center
                     border flex-shrink-0"
              :style="{ borderColor: classColor(char.classe) }"
          >
            <img
                v-if="classImage(char.classe)"
                :src="classImage(char.classe)"
                :alt="char.classe"
                class="w-full h-full object-cover"
            />
            <span v-else class="text-[11px] font-bold" :style="{ color: classColor(char.classe) }">
              {{ char.pseudo.slice(0, 2).toUpperCase() }}
            </span>
          </div>

          <div class="flex-1 min-w-0">
            <p class="text-[13px] font-bold text-[#c8e8f0] truncate">{{ char.pseudo }}</p>
            <p class="text-[11px] text-[#4a7a8a]">{{ char.classe }} · Init. {{ char.initiative }}</p>
          </div>

          <div class="flex gap-1 flex-wrap justify-end max-w-[160px]">
            <span v-for="role in char.roles" :key="role"
                  class="text-[9px] font-bold px-1.5 py-0.5 rounded bg-[#0f2a36]
                         text-[#1eb8cc] border border-[#1eb8cc22]">
              {{ role }}
            </span>
          </div>

          <button
              @click.stop="deleteCharacter(char.id)"
              class="text-[#2a5060] hover:text-[#ff6b6b] transition-colors text-sm ml-1"
          >
            ✕
          </button>
        </div>
      </div>
    </div>

    <transition name="slide">
      <div v-if="showForm"
           class="w-72 bg-[#091820] border-l border-[#0f2a36] flex flex-col overflow-y-auto">

        <div class="px-5 py-4 border-b border-[#0f2a36] flex items-center justify-between">
          <span class="text-[13px] font-bold text-[#e0f0f5]">
            {{ editingChar ? 'Modifier' : 'Nouveau personnage' }}
          </span>
          <button @click="showForm = false" class="text-[#2a5060] hover:text-[#e0f0f5]">✕</button>
        </div>

        <div class="flex flex-col gap-4 px-5 py-4">

          <!-- Aperçu avatar -->
          <div class="flex justify-center">
            <div
                class="w-16 h-16 rounded-full overflow-hidden border-2 flex items-center justify-center"
                :style="{ borderColor: classColor(form.classe) }"
            >
              <img
                  v-if="form.classe && classImage(form.classe)"
                  :src="classImage(form.classe)"
                  :alt="form.classe"
                  class="w-full h-full object-cover"
              />
              <span v-else class="text-lg font-bold" :style="{ color: classColor(form.classe) || '#4a7a8a' }">
                {{ form.pseudo ? form.pseudo.slice(0, 2).toUpperCase() : '?' }}
              </span>
            </div>
          </div>

          <!-- Pseudo -->
          <div>
            <label class="text-[10px] font-bold text-[#4a7a8a] uppercase tracking-wider">Pseudo</label>
            <input
                v-model="form.pseudo"
                placeholder="Nom du personnage"
                class="mt-1 w-full bg-[#0d1f26] border border-[#1a3a4a] rounded px-3 py-2
                     text-[13px] text-[#e0f0f5] outline-none focus:border-[#1eb8cc]
                     transition-colors placeholder-[#2a5060]"
            />
          </div>

          <!-- Classe -->
          <div>
            <label class="text-[10px] font-bold text-[#4a7a8a] uppercase tracking-wider">Classe</label>
            <select
                v-model="form.classe"
                class="mt-1 w-full bg-[#0d1f26] border border-[#1a3a4a] rounded px-3 py-2
                     text-[13px] text-[#e0f0f5] outline-none focus:border-[#1eb8cc] transition-colors"
            >
              <option value="">Sélectionner...</option>
              <option v-for="c in classes" :key="c" :value="c">{{ c }}</option>
            </select>
          </div>

          <!-- Éléments -->
          <div>
            <label class="text-[10px] font-bold text-[#4a7a8a] uppercase tracking-wider mb-2 block">
              Éléments
            </label>
            <div class="flex flex-wrap gap-1.5">
              <button
                  v-for="el in elements" :key="el.name"
                  @click="toggleSelection(form.elements, el.name)"
                  class="px-2.5 py-1 rounded text-[11px] font-bold border transition-all"
                  :style="form.elements.includes(el.name)
                    ? { background: el.color + '33', borderColor: el.color, color: el.color }
                    : { background: 'transparent', borderColor: '#1a3a4a', color: '#4a7a8a' }"
              >
                {{ el.name }}
              </button>
            </div>
          </div>

          <!-- Rôles -->
          <div>
            <label class="text-[10px] font-bold text-[#4a7a8a] uppercase tracking-wider mb-2 block">
              Rôles
            </label>
            <div class="flex flex-wrap gap-1.5">
              <button
                  v-for="role in roles" :key="role"
                  @click="toggleSelection(form.roles, role)"
                  class="px-2.5 py-1 rounded text-[11px] font-bold border transition-all"
                  :class="form.roles.includes(role)
                    ? 'bg-[#1eb8cc33] border-[#1eb8cc] text-[#1eb8cc]'
                    : 'bg-transparent border-[#1a3a4a] text-[#4a7a8a] hover:border-[#2a5a6a]'"
              >
                {{ role }}
              </button>
            </div>
          </div>

          <!-- Initiative -->
          <div>
            <label class="text-[10px] font-bold text-[#4a7a8a] uppercase tracking-wider">Initiative</label>
            <input
                v-model.number="form.initiative"
                type="number"
                min="0"
                placeholder="0"
                class="mt-1 w-full bg-[#0d1f26] border border-[#1a3a4a] rounded px-3 py-2
                     text-[13px] text-[#e0f0f5] outline-none focus:border-[#1eb8cc]
                     transition-colors placeholder-[#2a5060]"
            />
          </div>

        </div>

        <div class="mt-auto px-5 py-4 border-t border-[#0f2a36]">
          <button
              @click="saveCharacter"
              :disabled="!form.pseudo"
              class="w-full bg-[#1eb8cc] text-[#091820] font-bold text-[11px] tracking-widest
                   rounded-full py-2 hover:bg-[#28d4e8] transition-colors disabled:opacity-40
                   disabled:cursor-not-allowed"
          >
            {{ editingChar ? 'Mettre à jour' : 'Créer' }}
          </button>
        </div>

      </div>
    </transition>

  </div>
</template>

<script setup lang="ts">
import { load } from '@tauri-apps/plugin-store'
import { CLASSES, classColor, classImage } from '~/composables/useCharacters'
import type { Character } from '~/composables/useCharacters'

const classes = Object.keys(CLASSES).sort()

const elements = [
  { name: 'Feu',   color: '#F0997B' },
  { name: 'Eau',   color: '#85B7EB' },
  { name: 'Air',   color: '#9FE1CB' },
  { name: 'Terre', color: '#C0DD97' },
  { name: 'Multi', color: '#AFA9EC' },
]

const roles = [
  'Soins', 'Dégâts', 'Amélioration', 'Invocation',
  'Entrave', 'Placement', 'Protection', 'Tank',
]

const characters = ref<Character[]>([])
const showForm = ref(false)
const editingChar = ref<Character | null>(null)

const emptyForm = () => ({
  pseudo: '',
  classe: '',
  elements: [] as string[],
  roles: [] as string[],
  initiative: 0,
})

const form = ref(emptyForm())

async function getStore() {
  return await load('settings.json')
}

onMounted(async () => {
  const store = await getStore()
  const saved = await store.get<Character[]>('characters')
  if (saved && saved.length > 0) characters.value = saved
})

function openForm(char?: Character) {
  if (char) {
    editingChar.value = char
    form.value = { ...char, elements: [...char.elements], roles: [...char.roles] }
  } else {
    editingChar.value = null
    form.value = emptyForm()
  }
  showForm.value = true
}

function toggleSelection(arr: string[], item: string) {
  const idx = arr.indexOf(item)
  if (idx === -1) arr.push(item)
  else arr.splice(idx, 1)
}

async function saveCharacter() {
  if (!form.value.pseudo) return
  const store = await getStore()

  if (editingChar.value) {
    const idx = characters.value.findIndex(c => c.id === editingChar.value!.id)
    if (idx !== -1) characters.value[idx] = { ...form.value, id: editingChar.value.id }
  } else {
    characters.value.push({ ...form.value, id: crypto.randomUUID() })
  }

  await store.set('characters', characters.value)
  await store.save()
  showForm.value = false
}

async function deleteCharacter(id: string) {
  characters.value = characters.value.filter(c => c.id !== id)
  const store = await getStore()
  await store.set('characters', characters.value)
  await store.save()
}
</script>

<style scoped>
.slide-enter-active, .slide-leave-active {
  transition: transform 0.2s ease;
}
.slide-enter-from, .slide-leave-to {
  transform: translateX(100%);
}
</style>