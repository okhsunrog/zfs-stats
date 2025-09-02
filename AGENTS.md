# AGENTS.md — ZFS Statistics Dashboard

This document is a concise playbook for AI agents (and humans) developing this ZFS Statistics Dashboard built with Tauri + Vue. It covers the project layout, core workflows, ZFS-specific features, and "gotchas" to keep changes safe, consistent, and shippable.

## Project Overview

This is a desktop application that provides a beautiful, real-time dashboard for monitoring ZFS storage systems. The app executes `zfs list -t all -j` commands and presents the data in an intuitive UI with detailed statistics, usage graphs, and management views.

## TL;DR Commands (Bun)

- Install: `bun install` (resolves and installs dependencies)
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
  - Theme toggle: `src/components/common/ThemeToggle.vue` (DaisyUI Theme Controller)
  - Specta bindings: `src/bindings.ts` (generated during dev)

- Backend (Tauri 2 + Rust)
  - `src-tauri/src/lib.rs`: builds the Tauri app, registers Specta commands/events, initializes plugins
  - `src-tauri/src/commands/mod.rs`: ZFS commands (`get_zfs_stats`) and utilities, plus example commands
  - `src-tauri/src/logging.rs`: tracing subscriber for stdout logging
  - Plugins enabled: dialog, fs, log, os
  - Dependencies: tokio (for async process execution), serde/serde_json (for ZFS JSON parsing)
  - Capabilities (ACL): `src-tauri/capabilities/default.json`
  - Config (JSON5): `src-tauri/tauri.conf.json5` (icons, dev hooks)

## ZFS-Specific Architecture

### Core ZFS Command
The main ZFS functionality is implemented in `get_zfs_stats()` command which:
1. Executes `zfs list -t all -j` using tokio::process::Command
2. Parses the JSON output into structured Rust types
3. Organizes data into pools, filesystems, snapshots, and bookmarks
4. Calculates totals and usage statistics
5. Returns organized data to the frontend

### Frontend Components
- `src/stores/zfsStore.ts`: Pinia store managing ZFS state and data processing
- `src/components/zfs/ZfsDashboard.vue`: Main dashboard with overview cards and pool tabs
- `src/components/zfs/PoolDetails.vue`: Detailed view of filesystems, snapshots, and bookmarks per pool

### Data Flow
1. Frontend calls `commands.getZfsStats()` from bindings
2. Rust executes ZFS command and parses JSON
3. Data is returned as structured types to Vue store
4. Store processes data for UI consumption (size parsing, percentage calculations)
5. Components reactively display organized stats with usage graphs and tables

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
        commands::get_zfs_stats,
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

## Logging

- Use `tracing` macros (`trace!`, `debug!`, `info!`, `warn!`, `error!`).
- Logs are output to stdout for debugging and development.


## Permissions & Capabilities

- Edit `src-tauri/capabilities/default.json` to grant only what’s needed (dialog/fs/log/os are enabled by default here). Keep least-privilege in mind.
- If you add a new plugin, mirror JS and Rust versions and extend capabilities accordingly.

## Theming (DaisyUI v5)

- Themes: `nord` (light), `dim` (dark). Default is applied pre-mount (see `index.html`).
- The Vite plugin injects DaisyUI with the allowed themes based on `src/constants/themes.ts`.

- Latest DaisyUI docs for LLMs: https://daisyui.com/llms.txt

## Version Compatibility (Important)

- Keep plugin versions aligned between NPM and Rust crates (same major/minor). Examples:
  - `@tauri-apps/plugin-log` v2.7.x ↔ `tauri-plugin-log` 2.7.x
  - `@tauri-apps/plugin-fs` v2.4.x ↔ `tauri-plugin-fs` 2.4.x
  - `@tauri-apps/plugin-os` v2.3.x ↔ `tauri-plugin-os` 2.3.x
  - `@tauri-apps/plugin-dialog` v2.4.x ↔ `tauri-plugin-dialog` 2.4.x

## Script Reference (Bun)

- `bun install` – Resolves and installs dependencies from `bun.lock`
- `bun tauri dev` – Dev (Vite + Tauri, generates Specta bindings)
- `bun run dev:test` – Test dev server compilation and auto-stop when ready
- `bun tauri build` – Build installers/bundles
- `bun type-check` – `vue-tsc` project check
- `bun lint` / `bun lint:check` – ESLint
- `bun format` / `bun format:check` – Prettier

## Security Notes

- Current CSP is `null` to ease development; consider tightening for production.
- FS plugin permissions include `fs:allow-write-text-file`; restrict as needed.

## ZFS System Requirements

- **ZFS Installation**: The system must have ZFS installed and the `zfs` command available in PATH
- **Permissions**: The application needs read access to ZFS datasets (typically requires running as root or with appropriate sudo permissions)
- **Platform Support**: Primarily designed for Linux systems with ZFS, but should work on any Unix-like system with ZFS

## ZFS Data Structure Reference

See `ZFS_DATA_STRUCTURE.md` for detailed documentation of the JSON output structure from `zfs list -t all -j`.

Key data types:
- **Filesystems**: Mountable datasets with `used`/`available` space
- **Snapshots**: Point-in-time backups with `@snapshot_name` suffix
- **Bookmarks**: Replication markers with `#bookmark_name` suffix

## When Extending the ZFS App

- **New ZFS Commands**: Add to `src-tauri/src/commands/mod.rs` and register in `lib.rs`
- **UI Components**: Place ZFS-specific components in `src/components/zfs/`
- **Data Processing**: Extend the Pinia store in `src/stores/zfsStore.ts`
- **Size Parsing**: Use existing helper functions for consistent size formatting
- **Error Handling**: ZFS commands can fail due to permissions or missing datasets
- **Regenerate bindings** by running the dev task after Rust changes
- Keep UI theming in sync with DaisyUI variables; extend `themes.ts` if adding themes
- Avoid mixing package managers; use Bun only
- Do not introduce i18n unless explicitly requested

## Minimal Validation Checklist

1) `bun tauri dev` launches (no version mismatch errors)
2) Theme loads as `nord` by default; toggle to `dim` works
3) ZFS dashboard loads and displays stats (if ZFS is available)
4) Refresh button works and updates data
5) Pool tabs switch correctly and show filesystem/snapshot details
6) `bun lint:check`, `bun format:check`, and `bun type-check` pass
7) `bun tauri build` succeeds

## ZFS Testing Notes

- **Development without ZFS**: The app will show an error if ZFS is not available, which is expected behavior
- **Mock Data**: Consider adding a mock/demo mode for development on systems without ZFS
- **Permissions**: Test with appropriate permissions to access ZFS datasets
- **Error Handling**: Verify graceful error handling when ZFS commands fail
