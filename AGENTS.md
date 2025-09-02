# AGENTS.md — Working Effectively With This Template

This document is a concise playbook for AI agents (and humans) developing apps with this Tauri + Vue template. It covers the project layout, core workflows, and “gotchas” to keep changes safe, consistent, and shippable.

## TL;DR Commands (Bun)

- Dev: `bun tauri dev` (runs Vite + Tauri, generates Specta bindings, streams logs)
- Build: `bun tauri build` (packs installers/bundles)
- Lint/Format/Types:
  - `bun lint`, `bun lint:check`
  - `bun format`, `bun format:check`
  - `bun type-check`

Use Bun for all package and script tasks. Do not mix with npm/yarn/pnpm.

## Architecture Overview

- Frontend (Vue 3 + Vite + TS)
  - State: Pinia with persisted state
  - UI styling: Tailwind v4 + DaisyUI v5 (themes: `nord` and `dim`)
  - Terminal: xterm.js component (`src/components/terminal/TerminalDisplay.vue`) that listens to Rust logs
  - Theme toggle: `src/components/common/ThemeToggle.vue` (DaisyUI Theme Controller)
  - Specta bindings: `src/bindings.ts` (generated during dev)

- Backend (Tauri 2 + Rust)
  - `src-tauri/src/lib.rs`: builds the Tauri app, registers Specta commands/events, initializes plugins
  - `src-tauri/src/commands/mod.rs`: example commands (`greet`, `emit_test_logs`) annotated for Specta
  - `src-tauri/src/logging.rs`: tracing subscriber that forwards logs as `log-event` to the frontend
  - Plugins enabled: dialog, fs, log, os
  - Capabilities (ACL): `src-tauri/capabilities/default.json`
  - Config (JSON5): `src-tauri/tauri.conf.json5` (icons, dev hooks, android block)

## Adding a New Rust Command (with Specta)

1) Implement in `src-tauri/src/commands/mod.rs`:

```rust
#[tauri::command]
#[specta::specta]
pub async fn do_something(arg: String) -> Result<String, String> {
    tracing::info!("do_something called: {}", arg);
    Ok(format!("processed: {}", arg))
}
```

2) Register it in `src-tauri/src/lib.rs`:

```rust
let specta_builder = tauri_specta::Builder::<tauri::Wry>::new()
    .commands(tauri_specta::collect_commands![
        commands::greet,
        commands::emit_test_logs,
        commands::do_something,
    ])
    .events(tauri_specta::collect_events![logging::LogEvent]);
```

3) Run `bun tauri dev` to regenerate `src/bindings.ts` and use it on the frontend via `import { commands } from '@/bindings'` (or call via `invoke('do_something', { arg })`).

## Adding an Event

1) Define a serializable type and derive Specta Event in Rust:

```rust
#[derive(Debug, Clone, serde::Serialize, specta::Type, tauri_specta::Event)]
pub struct MyEvent { pub message: String }
```

2) Add it to `.events(collect_events![...])` in `lib.rs` and emit via `app_handle.emit("my-event", MyEvent { ... })`.

3) After `bun tauri dev`, consume in FE via Specta bindings: `events.myEvent.listen(cb)`.

## Logging → Terminal

- Use `tracing` macros (`trace!`, `debug!`, `info!`, `warn!`, `error!`).
- Logs are forwarded to the frontend as `log-event` (see `logging.rs`).
- Terminal colors are aligned with DaisyUI v5 (themes `nord`/`dim`).
- There is a sample button in the UI to emit test logs.

## Permissions & Capabilities

- Edit `src-tauri/capabilities/default.json` to grant only what’s needed (dialog/fs/log/os are enabled by default here). Keep least-privilege in mind.
- If you add a new plugin, mirror JS and Rust versions and extend capabilities accordingly.

## Theming (DaisyUI v5)

- Themes: `nord` (light), `dim` (dark). Default is applied pre-mount (see `index.html`).
- The Vite plugin injects DaisyUI with the allowed themes based on `src/constants/themes.ts`.
- Terminal theme recalculates from CSS variables on theme changes (MutationObserver).
- Latest DaisyUI docs for LLMs: https://daisyui.com/llms.txt

## Version Compatibility (Important)

- Keep plugin versions aligned between NPM and Rust crates (same major/minor). Examples:
  - `@tauri-apps/plugin-log` v2.7.x ↔ `tauri-plugin-log` 2.7.x
  - `@tauri-apps/plugin-fs` v2.4.x ↔ `tauri-plugin-fs` 2.4.x
  - `@tauri-apps/plugin-os` v2.3.x ↔ `tauri-plugin-os` 2.3.x
  - `@tauri-apps/plugin-dialog` v2.4.x ↔ `tauri-plugin-dialog` 2.4.x

## Script Reference (Bun)

- `bun tauri dev` – Dev (Vite + Tauri, generates Specta bindings)
- `bun tauri build` – Build installers/bundles
- `bun type-check` – `vue-tsc` project check
- `bun lint` / `bun lint:check` – ESLint
- `bun format` / `bun format:check` – Prettier

## Security Notes

- Current CSP is `null` to ease development; consider tightening for production.
- FS plugin permissions include `fs:allow-write-text-file`; restrict as needed.

## When Extending the Template

- Prefer adding new commands under `src-tauri/src/commands/` and registering in `lib.rs`.
- Regenerate bindings by running the dev task after changes.
- Keep UI theming in sync with DaisyUI variables; extend `themes.ts` if adding themes.
- Avoid mixing package managers; use Bun only.
- Do not introduce i18n unless explicitly requested; if needed, follow the `fwupd-gui` pattern (vue-i18n + unplugin-vue-i18n) and wire it in `vite.config.ts`.

## Minimal Validation Checklist

1) `bun tauri dev` launches (no version mismatch errors)
2) Theme loads as `nord` by default; toggle to `dim` works
3) Terminal displays startup logs and test logs
4) `bun lint:check`, `bun format:check`, and `bun type-check` pass
5) `bun tauri build` succeeds
