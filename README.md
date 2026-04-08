# Dofus Switcher

Application bureau Windows permettant de switcher rapidement entre plusieurs clients Dofus ouverts simultanément, via des raccourcis souris ou clavier configurables.

---

## Fonctionnalités

- Détection automatique des fenêtres Dofus ouvertes
- Switch entre les clients via Mouse4 / Mouse5 (ou touches clavier configurables)
- Clic sur un compte dans l'interface pour le mettre au premier plan
- Fonctionne en mode fenêtré et plein écran
- Interface dans le style du launcher Dofus officiel

---

## Téléchargement

Rendez-vous sur la page [Releases](../../releases) pour télécharger le dernier installeur Windows (`.exe`).

---

## Stack technique

| Couche | Technologie |
|--------|-------------|
| UI | Nuxt 4 + Vue 3 + Tailwind CSS |
| Desktop | Tauri 2 |
| Backend natif | Rust + crate `windows` |
| Accès Win32 | `EnumWindows`, `SetForegroundWindow`, `AttachThreadInput`, `WH_MOUSE_LL` |

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

## Installation du projet
```bash
# Cloner le dépôt
git clone https://github.com/TON_USERNAME/dofus-switcher.git
cd dofus-switcher

# Installer les dépendances Node
npm install
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

Génère deux fichiers dans `src-tauri/target/release/bundle/` :
- `nsis/dofus-switcher_x.x.x_x64-setup.exe` — installeur Windows
- `dofus-switcher.exe` — exécutable portable

---

## Structure du projet

```
dofus-switcher/
├── app/
│   ├── assets/css/        # Styles globaux Tailwind
│   ├── components/        # Composants Vue réutilisables
│   │   ├── AppSidebar.vue
│   │   └── AccountRow.vue
│   └── pages/             # Vues de l'application
│       ├── index.vue      # Multi-compte (liste des clients)
│       ├── hotkeys.vue    # Configuration des touches
│       └── settings.vue   # Options
├── src-tauri/
│   ├── src/
│   │   ├── main.rs        # Point d'entrée Tauri + commandes
│   │   ├── windows_api.rs # Détection et focus des fenêtres Win32
│   │   └── mouse_hook.rs  # Hook souris/clavier bas niveau
│   ├── Cargo.toml         # Dépendances Rust
│   └── tauri.conf.json    # Configuration Tauri
├── nuxt.config.ts         # Configuration Nuxt
└── package.json
```

---

## Publier une release

1. Mettre à jour la version dans `src-tauri/tauri.conf.json` et `src-tauri/Cargo.toml`
2. Lancer `npx tauri build`
3. Créer une release GitHub et uploader le `.exe` généré

---

## Compatibilité

- Windows 10 / 11 (x64)
- Dofus 3+