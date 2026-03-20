# CC Switch CLI - Agent Knowledge Base

**Generated:** 2026-03-20
**Type:** Rust CLI (Cargo, edition 2021)
**Size:** 11 source files (~700 LOC)

## Overview

CLI tool for managing CC Switch AI provider configurations. Reads/writes SQLite database at `~/.cc-switch/cc-switch.db`. Supports dual output modes: human-readable (ANSI tables) and AI-friendly (XML).

## Project Structure

```
ccswitch-cli/
├── Cargo.toml          # Dependencies: clap, anyhow, rusqlite, serde, tabled
├── src/
│   ├── main.rs         # Entry point, CLI parsing, error dispatch
│   ├── commands/       # list, switch, current, health commands
│   │   ├── mod.rs
│   │   ├── list.rs
│   │   ├── switch.rs
│   │   ├── current.rs
│   │   └── health.rs
│   ├── db/             # SQLite layer
│   │   ├── mod.rs      # Connection, path resolution
│   │   └── models.rs    # Provider, ProviderHealth structs + queries
│   └── formatter/      # Output formatters
│       ├── mod.rs
│       ├── human.rs     # ANSI table output using tabled
│       └── ai.rs        # XML output for AI agents
└── docs/PLAN.md
```

## Build & Run Commands

```bash
# Build (debug)
cargo build

# Build (release - aggressive optimization)
cargo build --release

# Run
cargo run -- list claude
cargo run -- switch claude --provider openrouter --dry-run
cargo run -- --ai health opencode

# Run single test
cargo test test_name_here

# Run all tests
cargo test

# Lint with clippy
cargo clippy --all-targets

# Format
cargo fmt

# Check formatting
cargo fmt -- --check
```

## Code Style Guidelines

### Imports & Module Structure
- Group imports: std → external (clap, anyhow, rusqlite, serde) → local (`crate::`)
- Use `use` for re-exports; qualify full paths in function bodies for clarity
- Modules use `mod.rs` for submodules; flat structure in `commands/` and `formatter/`

### Error Handling
- **Use `anyhow::Result<()>` for command handlers** — propagates errors to `main`
- **Use `anyhow::bail!("message")` for early returns** with error context
- **Use `anyhow::Context` for fallible operations** — `.context("description")`
- **Never use `unwrap()` or `expect()` in command code** — propagate instead
- Error output: XML in AI mode (`<ccswitch error="...">`), stderr in human mode

### Naming Conventions
| Item | Convention | Example |
|------|------------|---------|
| Modules | lowercase, snake_case | `db`, `formatter` |
| Structs | PascalCase | `Provider`, `ProviderHealth` |
| Functions | snake_case | `get_current`, `format_switch_result` |
| Enums | PascalCase | `Commands` (clap) |
| Variables | snake_case | `dry_run`, `provider_id` |
| Constants | SCREAMING_SNAKE_CASE | (none currently) |

### Struct & Function Signatures
- Command execute functions: `pub fn execute(app: &str, ai_mode: bool) -> Result<()>`
- DB query functions: `pub fn get_current(conn: &Connection, app_type: &str) -> Result<Option<Self>>`
- Formatter functions: `pub fn format_*(...) -> String`
- Prefer `&str` over `String` for input parameters; clone only when needed

### Formatting Rules
- **No rustfmt.toml** — use default Rust formatting
- 4-space indent, no tabs
- Max line length: let formatter decide (default ~100)
- Use `rustfmt` before committing: `cargo fmt`

### Serialization
- Use `serde` with `#[derive(Serialize, Deserialize)]` for JSON/JSON data
- Use `quick-xml` for XML serialization in AI mode
- Use `tabled::Table` for human-readable tables

### Database Patterns
- Connection passed as reference: `conn: &Connection`
- Use `params![]` macro for parameterized queries
- SQLite booleans as `i32` (0/1)
- JSON fields stored as `String` and parsed at read time

## Anti-Patterns

- **DO NOT** use `unwrap()` in command/formatter code — return `Result` instead
- **DO NOT** write to stdout in DB layer — keep it pure
- **DO NOT** mix human and AI output in same formatter — separate modules
- **DO NOT** commit without running `cargo fmt` and `cargo clippy`

## Dual Output Mode

Every command supports two output modes:
- **Human mode** (default): Uses `tabled` for ASCII tables, colored status icons (● ✓ ✗)
- **AI mode** (`--ai` flag): XML output with `<ccswitch>` root element

Formatter modules are strictly separated — no shared logic between human.rs and ai.rs.

## Database Schema (Read-Only Operations)

The CLI currently only reads from the CC Switch SQLite database:
- `~/.cc-switch/cc-switch.db`
- Tables: `providers`, `provider_health`, `settings`
- DB path resolved via `dirs::home_dir()` + `.cc-switch/cc-switch.db`

## Gotchas

- Database may not exist on first install — handle with clear error message
- `Provider::get_current` returns `Option<Self>` — handle `None` case
- Health data may be missing for new providers — use `Option<ProviderHealth>`
- AI XML output uses `escape_xml()` to sanitize user content
- Clap global `--version` setting enabled; version from `Cargo.toml`
