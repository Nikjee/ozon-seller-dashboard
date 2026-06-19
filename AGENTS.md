# Ozon Dashboard

**Generated:** 2026-06-16
**Stack:** Vite + Vue 3 + Tauri 2.0 (Rust) + Naive UI
**Lang:** JavaScript (no TS) — Rust backend

## STRUCTURE

```
src/                     # Vue 3 frontend (Composition API, SFC)
├── components/          # 14 flat UI components, naive-ui + Catppuccin
├── composables/         # 8 data-access composables (useXxx pattern)
├── styles/              # 2 theme/style files
└── main.js / App.vue    # Entry + root component
src-tauri/src/           # Rust backend — 8 modules (Ozon API, data, Tauri)
server/                  # Node server helpers (4 files)
```

## WHERE TO LOOK

| Task | Path | Notes |
|------|------|-------|
| UI components | `src/components/` | `ProductTreeTable` (data-table), `AnalyticsDashboard`, `ProductRow` |
| Data fetching | `src/composables/` | `useAnalyticsDashboard`, `useStocksAnalytics`, `useDashboard` |
| API integration | `src-tauri/src/` | `ozon.rs`, `analytics.rs`, `dashboard.rs`, `stocks.rs` |
| i18n | `src/composables/useI18n.js` | ru/en with flat key map, `t()` resolver |
| Config | `vite.config.js`, `tauri.conf.json` | Port 1420, native auto-import |

## CONVENTIONS

- **Vue**: Composition API `<script setup>` only, PascalCase components
- **Backend**: Rust `async fn` per Tauri command, serde_json::Value for dynamic shapes
- **Naming**: camelCase JS, snake_case Rust, CamelCase Components
- **Imports**: naive-ui components manually imported (no AutoImport plugin)
- **Styling**: Catppuccin CSS vars via naive-ui darkThemeOverrides, BEM-like class names
- **Router**: none — `n-tabs` + manual `v-if`/`lazy-mount` switching
- **Icons**: naive-ui built-in icons only

## ANTI-PATTERNS (THIS PROJECT)

- **No `as any`, `@ts-ignore`, empty catch** — codebase is clean, keep it so
- **No Vue Router** — don't add it, n-tabs switching is intentional
- **No runtime type guards** — pure JS, rely on API shape assumptions
- **No direct DOM manipulation** — use Vue reactivity

## COMMANDS

```bash
npm run dev          # Vite dev server (port 1420)
npm run tauri dev    # Tauri desktop app
cd src-tauri && cargo build   # Rust compilation check
npm run test:unit    # Vitest suite
```

## NOTES

- Tests: 5 specs (vitest + jsdom), covers components and composables
- CI: GitHub Actions 3-platform matrix (no test step — only build)
- Data: Ozon Seller API via Tauri IPC commands, no HTTP layer in frontend
- Ports: dev server on 1420, Tauri webview connects automatically
