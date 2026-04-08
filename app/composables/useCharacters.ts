import { load } from '@tauri-apps/plugin-store'

export interface Character {
    id: string
    pseudo: string
    classe: string
    elements: string[]
    roles: string[]
    initiative: number
}

export const CLASSES: Record<string, { color: string; image: string }> = {
    'Cra':          { color: '#97C459', image: '/classes/cra.jpg' },
    'Ecaflip':      { color: '#D4537E', image: '/classes/ecaflip.jpg' },
    'Eliotrope':    { color: '#7F77DD', image: '/classes/eliotrop.jpg' },
    'Eniripsa':     { color: '#9FE1CB', image: '/classes/eniripsa.jpg' },
    'Enutrof':      { color: '#EF9F27', image: '/classes/enutrof.jpg' },
    'Feca':         { color: '#378ADD', image: '/classes/feca.jpg' },
    'Forgelance':   { color: '#85B7EB', image: '/classes/forgelance.jpg' },
    'Huppermage':   { color: '#5DCAA5', image: '/classes/huppermage.jpg' },
    'Iop':          { color: '#E24B4A', image: '/classes/iop.jpg' },
    'Osamodas':     { color: '#639922', image: '/classes/osamodas.jpg' },
    'Ouginak':      { color: '#BA7517', image: '/classes/ouginak.jpg' },
    'Pandawa':      { color: '#BA7517', image: '/classes/pandawa.jpg' },
    'Roublard':     { color: '#888780', image: '/classes/roublard.jpg' },
    'Sacrieur':     { color: '#D85A30', image: '/classes/sacrieur.jpg' },
    'Sadida':       { color: '#3B6D11', image: '/classes/sadida.jpg' },
    'Sram':         { color: '#444441', image: '/classes/sram.jpg' },
    'Steamer':      { color: '#FAC775', image: '/classes/steamer.jpg' },
    'Xelor':        { color: '#AFA9EC', image: '/classes/xelor.jpg' },
    'Zobal':        { color: '#ED93B1', image: '/classes/zobal.jpg' },
}

export function classColor(classe: string): string {
    return CLASSES[classe]?.color ?? '#1eb8cc'
}

export function classImage(classe: string): string {
    return CLASSES[classe]?.image ?? ''
}

export async function getCharacters(): Promise<Character[]> {
    const store = await load('settings.json')
    return await store.get<Character[]>('characters') ?? []
}

export function extractPseudo(windowTitle: string): string {
    return windowTitle.split(' - ')[0]?.trim() ?? windowTitle
}

export function matchCharacter(windowTitle: string, characters: Character[]): Character | null {
    const pseudo = extractPseudo(windowTitle)
    return characters.find(c =>
        c.pseudo.toLowerCase() === pseudo.toLowerCase()
    ) ?? null
}