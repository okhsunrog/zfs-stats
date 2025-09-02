# AGENTS.md — ZFS Statistics Dashboard (Axum Web Branch)

This document is a concise playbook for AI agents (and humans) developing this ZFS Statistics Dashboard built with Vue + Axum. It covers the project layout, core workflows, ZFS-specific features, and "gotchas" to keep changes safe, consistent, and shippable.

## Project Overview

This branch migrates the app from Tauri to a pure web architecture: a Vue 3 frontend built with Vite and an Axum (Rust) backend that serves the built assets from an embedded bundle and exposes a `/api/zfs` endpoint. Desktop/Tauri content in this document is not applicable for this branch.

## TL;DR Commands (Bun)

- Install: `bun install` (resolves and installs dependencies)
- Dev (FE): `bun run dev`
- Build (FE): `bun run build`
- Run server: `bun run server`
- Export TS types from Rust: `bun run types:export`
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

- Backend (Axum + Rust)
  - `src/main.rs`: Axum server (serves embedded `dist/`, exposes `/api/zfs`)
  - `src/types.rs`: Shared ZFS types (`serde` + optional `specta::Type`)
  - `src/bin/export_types.rs`: Exports TS types to `src/bindings.ts`
  - Dependencies: tokio, axum, rust-embed, serde/serde_json

## ZFS-Specific Architecture

### Core ZFS Command
The main ZFS functionality is implemented in the server `get_zfs_stats()` handler which:
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
1. Frontend fetches `/api/zfs`
2. Rust executes ZFS command and parses JSON
3. Data is returned as structured types to Vue store
4. Store processes data for UI consumption (size parsing, percentage calculations)
5. Components reactively display organized stats with usage graphs and tables

## Adding/Updating Types with Specta (optional)

1) Ensure types derive `specta::Type` in `src/types.rs`.

2) Export to TS by running:

```
bun run types:export
```

This overwrites `src/bindings.ts` with types-only declarations.

## Events

This branch does not use Tauri events. If you need server push, consider Server-Sent Events (SSE) or WebSockets from Axum, and mirror types with Specta as needed.

## Logging

- Use `tracing` macros (`trace!`, `debug!`, `info!`, `warn!`, `error!`).
- Logs are output to stdout for debugging and development.


## Permissions

- Ensure the server process has permission to read ZFS datasets.

## Theming (DaisyUI v5)

- Themes: `nord` (light), `dim` (dark). Default is applied pre-mount (see `index.html`).
- The Vite plugin injects DaisyUI with the allowed themes based on `src/constants/themes.ts`.

- Latest DaisyUI docs for LLMs: https://daisyui.com/llms.txt

## Version Compatibility

- Keep Rust crate versions in `Cargo.toml` compatible. No Tauri plugins in this branch.

## Script Reference (Bun)

- `bun install` – Resolves and installs dependencies
- `bun run dev` – Vite dev server
- `bun run build` – Build frontend assets
- `bun run server` – Run Axum server (serves embedded assets)
- `bun type-check` – `vue-tsc` project check
- `bun lint` / `bun lint:check` – ESLint
- `bun format` / `bun format:check` – Prettier

## Security Notes

- Current CSP is `null` to ease development; consider tightening for production.

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

- **New API Endpoints**: Add routes in `src/main.rs` (or split into modules), return JSON.
- **UI Components**: Place ZFS-specific components in `src/components/zfs/`.
- **Data Processing**: Extend the Pinia store in `src/stores/zfsStore.ts`.
- **Size Parsing**: Use existing helper functions for consistent size formatting.
- **Error Handling**: ZFS commands can fail due to permissions or missing datasets.
- **Regenerate bindings** by running `bun run types:export` after Rust type changes.
- Keep UI theming in sync with DaisyUI variables; extend `themes.ts` if adding themes.
- Avoid mixing package managers; use Bun only.
- Do not introduce i18n unless explicitly requested.

## Minimal Validation Checklist

1) `bun run build` succeeds; `dist/` contains assets
2) `bun run server` starts and serves `http://localhost:8080`
3) Theme loads as `nord` by default; toggle to `dim` works
4) ZFS dashboard loads and displays stats (if ZFS is available)
5) Refresh button works and updates data
6) `bun lint:check`, `bun format:check`, and `bun type-check` pass

## ZFS Testing Notes

- **Development without ZFS**: The app will show an error if ZFS is not available, which is expected behavior
- **Mock Data**: Consider adding a mock/demo mode for development on systems without ZFS
- **Permissions**: Test with appropriate permissions to access ZFS datasets
- **Error Handling**: Verify graceful error handling when ZFS commands fail
