# ZFS Statistics Dashboard (Web)

A beautiful, real-time web application for monitoring ZFS storage systems. Built with Vue 3 + TypeScript for the UI and a Rust Axum server that embeds the built frontend into a single static binary.

![ZFS Stats Dashboard](https://via.placeholder.com/800x600/2563eb/ffffff?text=ZFS+Statistics+Dashboard)

## Features

- **Real-time ZFS Monitoring**: Live statistics from `zfs list -t all -j`
- **Beautiful UI**: Modern dashboard with DaisyUI themes (light/dark mode)
- **Detailed Views**: 
  - Pool overview with usage statistics
  - Filesystem details with mountpoints and space usage
  - Snapshot management and history
  - Bookmark tracking for replication
- **Interactive Charts**: Visual representation of storage usage
- **Cross-platform**: Works on Linux, macOS, and other Unix-like systems with ZFS

## Requirements

- **ZFS Installation**: System must have ZFS installed with `zfs` command available
- **Permissions**: Read access to ZFS datasets (may require root or sudo permissions)
- **Node.js/Bun**: For development and building
- **Rust**: For Axum server compilation

## Quick Start (Web)

```bash
# Clone the repository
git clone <repository-url>
cd zfs-stats

# Install dependencies
bun install

# Run in development mode
bun run types:export # optional (placeholder exists)

# Build for production (frontend)
bun run build

# Start the Axum server (serves embedded dist/)
bun run server
```

## Axum Web App (Embedded Assets)

This branch uses a standalone Axum web server that serves the built frontend from an embedded asset bundle and exposes the ZFS API over HTTP.

Steps:

1) Build the frontend (creates `dist/`):

```
bun run build
```

2) Run the Axum server (embeds `dist/` at compile time):

```
bun run server
```

3) Open the app in your browser:

```
http://localhost:8080
```

API endpoints:
- `GET /api/zfs` – Returns organized ZFS stats (pools, filesystems, snapshots, bookmarks, totals)

Notes:
- Re-run the server after rebuilding the frontend to re-embed updated assets.
- The server binds to `0.0.0.0:8080` by default. Configure via `HOST` and `PORT` env vars.
- Requires ZFS to be installed and accessible in PATH.

### Deployment

- Build the UI and server (assets embedded into the binary):
  - `bun run build` then `cargo build --manifest-path server/Cargo.toml --release`
- Static binary (default): builds for MUSL by default using server/.cargo/config.toml
  - Output binary: `server/target/x86_64-unknown-linux-musl/release/zfs-stats-web`

Copy to server and run:

```
scp server/target/x86_64-unknown-linux-musl/release/zfs-stats-web user@your-server:/opt/zfs-stats/
ssh user@your-server 'cd /opt/zfs-stats && HOST=0.0.0.0 PORT=8080 RUST_LOG=info ./zfs-stats-web'
```

Environment variables:
- `HOST`: bind address (default `0.0.0.0`)
- `PORT`: bind port (default `8080`)
- `RUST_LOG`: log filter (e.g. `info`, `debug`, `info,tower_http=info`)

### Static Binary (Linux)

Build a fully static Linux binary using MUSL:

```
rustup target add x86_64-unknown-linux-musl
RUSTFLAGS="-C target-feature=+crt-static" \
  cargo build --manifest-path server/Cargo.toml --release --target x86_64-unknown-linux-musl
```

Result: `server/target/x86_64-unknown-linux-musl/release/zfs-stats-web`

Notes:
- `server/.cargo/config.toml` sets `musl-gcc` as the linker for the MUSL target. Install `musl-tools` (Debian/Ubuntu) or equivalent.
- Logs print to stdout by default. Adjust verbosity via `RUST_LOG`, e.g. `RUST_LOG=info,tower_http=info`.

## Development

This project uses:
- **Frontend**: Vue 3 + TypeScript + Vite + DaisyUI + Tailwind CSS
- **Backend**: Rust (Axum + Tokio)
- **State Management**: Pinia
- **Type Safety**: Specta (optional) for generating TS types

### Commands

- `bun install` - Install dependencies
- `bun run dev` - Vite dev server (frontend only)
- `bun run build` - Build frontend (dist/)
- `bun run server` - Run Axum server (serves embedded dist/)
- `bun run types:export` - Generate `src/bindings.ts` from Rust types (Specta)
- `bun lint` / `bun lint:check` - ESLint
- `bun format` / `bun format:check` - Prettier
- `bun type-check` - TypeScript validation

### Project Structure

```
src/
├── components/
│   ├── zfs/
│   │   ├── ZfsDashboard.vue    # Main dashboard component
│   │   └── PoolDetails.vue     # Pool-specific details
│   └── common/
├── lib/
│   └── api.ts                  # HTTP client for /api/zfs
├── stores/
│   └── zfsStore.ts             # ZFS data management
└── bindings.ts                 # Types exported from Rust (Specta)

server/
├── src/
│   ├── main.rs                 # Axum server + embedded assets
│   ├── lib.rs                  # Exposes shared types module
│   ├── types.rs                # Shared ZFS types (serde + specta)
│   └── bin/export_types.rs     # Specta types exporter -> src/bindings.ts
└── Cargo.toml
```

## Architecture

### Data Flow

1. **Frontend** calls `commands.getZfsStats()` from generated bindings
2. **Rust backend** executes `zfs list -t all -j` using tokio::process::Command
3. **JSON parsing** converts ZFS output to structured Rust types
4. **Data processing** organizes pools, filesystems, snapshots, and bookmarks
5. **Vue components** reactively display organized statistics with charts and tables

### ZFS Data Structure

The application parses ZFS JSON output containing:
- **Filesystems**: Mountable datasets with usage statistics
- **Snapshots**: Point-in-time backups (names with `@`)
- **Bookmarks**: Replication markers (names with `#`)

See `ZFS_DATA_STRUCTURE.md` for detailed format documentation.

## Permissions

ZFS commands typically require elevated permissions. Configure your user or run the server with appropriate privileges to read datasets.

## Error Handling

The application gracefully handles:
- Missing ZFS installation
- Permission denied errors
- Invalid or corrupted ZFS data
- Network timeouts and system errors

## Themes

Supports DaisyUI themes:
- **Nord** (light mode) - default
- **Dim** (dark mode)

Toggle between themes using the theme switcher in the top-right corner.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Ensure all tests pass: `bun lint:check && bun format:check && bun type-check`
5. Submit a pull request

## License

[MIT License](LICENSE)

## Development Notes

- See `AGENTS.md` for detailed development guide (desktop content may refer to Tauri)
- TypeScript bindings can be generated from Rust types via Specta (`bun run types:export`)
- Keep UI components in `src/components/zfs/` for organization
- Extend the Pinia store for new data processing features
