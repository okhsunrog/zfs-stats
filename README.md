# ZFS Statistics Dashboard

A beautiful, real-time desktop application for monitoring ZFS storage systems. Built with Tauri, Rust, Vue 3, and TypeScript.

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
- **Rust**: For Tauri backend compilation

## Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd zfs-stats

# Install dependencies
bun install

# Run in development mode
bun tauri dev

# Build for production
bun tauri build
```

## Development

This project uses:
- **Frontend**: Vue 3 + TypeScript + Vite + DaisyUI + Tailwind CSS
- **Backend**: Tauri 2 + Rust + Tokio
- **State Management**: Pinia
- **Type Safety**: Specta for Rust ↔ TypeScript bindings

### Commands

- `bun install` - Install dependencies
- `bun tauri dev` - Start development server
- `bun tauri build` - Build production app
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
├── stores/
│   └── zfsStore.ts             # ZFS data management
└── bindings.ts                 # Auto-generated Rust bindings

src-tauri/
├── src/
│   ├── commands/
│   │   └── mod.rs              # ZFS commands and data structures
│   ├── lib.rs                  # App initialization
│   └── logging.rs              # Stdout logging
└── Cargo.toml                  # Rust dependencies
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

ZFS commands typically require elevated permissions. You may need to:

```bash
# Run with sudo (not recommended for GUI apps)
sudo bun tauri dev

# Or configure ZFS permissions for your user
# Add your user to appropriate groups or configure sudo rules
```

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

- See `AGENTS.md` for detailed development guide
- TypeScript bindings are auto-generated from Rust types
- Use `bun tauri dev` to regenerate bindings after Rust changes
- Keep UI components in `src/components/zfs/` for organization
- Extend the Pinia store for new data processing features