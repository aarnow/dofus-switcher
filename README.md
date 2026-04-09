# Dofus - Heros

Application bureau Windows permettant de switcher rapidement entre plusieurs clients Dofus ouverts simultanément, via des raccourcis souris ou clavier configurables.

> **100% externe au jeu** — Dofus - Heros ne modifie, n'injecte et n'interagit en aucune façon avec le client Dofus. Il se contente de détecter les fenêtres Windows ouvertes et d'en gérer l'ordre au premier plan, exactement comme vous le feriez manuellement avec Alt+Tab.

---

## Fonctionnalités

- Détection automatique des fenêtres Dofus ouvertes
- Switch entre les clients via Mouse4 / Mouse5 (ou touches clavier configurables)
- Tri automatique des comptes par initiative décroissante
- Profils de personnages : classe, éléments, rôles, initiative
- Association automatique entre fenêtres et profils via le pseudo
- Overlay discret affichant les comptes en haut de l'écran
- Clic sur un compte dans l'overlay ou l'interface pour le mettre au premier plan

---

## Téléchargement

Rendez-vous sur la page [Releases](../../releases) pour télécharger le dernier installeur Windows (`.exe`).

> **Note SmartScreen** : Windows peut afficher un avertissement de sécurité au lancement de l'installeur car l'application n'est pas encore signée numériquement. Cliquez sur "Informations complémentaires" puis "Exécuter quand même". Le code source est entièrement disponible sur ce dépôt pour vérification.

---

## Stack technique

| Couche | Technologie |
|--------|-------------|
| UI | Nuxt 4 + Vue 3 + Tailwind CSS |
| Desktop | Tauri 2 |
| Backend natif | Rust + crate `windows` |
| Accès Win32 | `EnumWindows`, `SetForegroundWindow`, `AttachThreadInput`, `WH_MOUSE_LL`, `WH_KEYBOARD_LL` |

---

## Prérequis développement

### Outils à installer

| Outil | Version minimale | Lien |
|-------|-----------------|------|
| Node.js | 18+ | https://nodejs.org |
| Rust | stable | https://rustup.rs |
| VS Build Tools (C++) | 2022 | https://visualstudio.microsoft.com/visual-cpp-build-tools/ |

> Lors de l'installation des VS Build Tools, cocher uniquement le workload **"Développement Desktop en C++"**.

### Installation de la Tauri CLI
```bash
npm install -g @tauri-apps/cli
```

---

## Commandes

### Lancer en mode développement
```bash
npx tauri dev
```

Lance le serveur Nuxt et la fenêtre Tauri simultanément.  
La première compilation Rust prend 3-5 minutes, les suivantes sont quasi instantanées.

### Compiler pour la production
```bash
npx tauri build
```

Génère l'installeur dans `src-tauri/target/release/bundle/nsis/` :
- `Dofus - Heros_0.1.0_x64-setup.exe` — installeur Windows

---

## Structure du projet

```
dofus-heros/
├── app/
│   ├── assets/css/          # Styles globaux Tailwind
│   ├── components/          # Composants Vue réutilisables
│   │   ├── AccountRow.vue   # Ligne de compte (page multi-compte)
│   │   ├── AccountRowSeparator.vue
│   │   ├── ActionButton.vue
│   │   ├── AppToggle.vue
│   │   ├── CharacterRow.vue # Ligne de personnage (page personnages)
│   │   ├── EmptyState.vue
│   │   ├── FormDivider.vue
│   │   ├── FormField.vue
│   │   ├── HotkeyBinding.vue
│   │   ├── SettingCard.vue
│   │   └── SettingRow.vue
│   ├── composables/
│   │   ├── useCapturing.ts  # État global capture de touche
│   │   └── useCharacters.ts # Données personnages, classes, rôles, éléments
│   ├── layouts/
│   │   └── default.vue      # Layout principal (topbar, navigation)
│   └── pages/
│       ├── index.vue        # Multi-compte (liste des clients)
│       ├── characters.vue   # Gestion des profils personnages
│       ├── settings.vue     # Options et configuration
│       └── overlay.vue      # Overlay always-on-top
├── src-tauri/
│   ├── src/
│   │   ├── main.rs          # Point d'entrée Tauri + commandes core
│   │   ├── overlay.rs       # Gestion de la fenêtre overlay
│   │   ├── system.rs        # Chemins, désinstallation, ouverture dossiers
│   │   ├── windows_api.rs   # Détection et focus des fenêtres Win32
│   │   └── mouse_hook.rs    # Hook souris/clavier bas niveau
│   ├── icons/               # Icônes générées par tauri icon
│   ├── Cargo.toml           # Dépendances Rust
│   └── tauri.conf.json      # Configuration Tauri
├── nuxt.config.ts           # Configuration Nuxt
└── package.json
```

---

## Compatibilité

- Windows 10 / 11 (x64) uniquement
- Dofus Unity