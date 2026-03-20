# CC Switch CLI - Plan & Blueprint

## Overview

A Rust CLI tool for managing CC Switch providers, designed for SSH users and AI agents.

## Problem Statement

CC Switch provides a GUI for managing AI coding CLI tool configurations, but SSH users cannot easily switch providers without GUI access.

## Solution

A lightweight CLI that reads/writes the CC Switch SQLite database directly.

## Current Status

### Implemented

- [x] Project structure (Rust + Clap)
- [x] Database layer (rusqlite)
- [x] Commands: `list`, `switch`, `current`, `health`
- [x] Dual output mode:
  - [x] Human mode (ANSI table output)
  - [x] AI mode (XML output, well-formed, properly escaped)
- [x] Shell completions (bash, elvish, fish, powershell, zsh via `completions` subcommand)
- [x] Validated app-type enum (claude, opencode, openclaw, codex, gemini)
- [x] DB indexes on `(app_type, sort_index)`, `(app_type, is_current)`, `(app_type, provider_id)` for health

### Pending

- [ ] GitHub repository creation
- [ ] Skill manifest for agent-skills
- [ ] Homebrew formula
- [ ] Binary releases (GitHub Actions)

## Command Reference

```bash
# List providers
ccswitch-cli list claude
ccswitch-cli list opencode

# Switch provider
ccswitch-cli switch claude --provider openrouter
ccswitch-cli switch opencode --provider anthropic --dry-run

# Show current
ccswitch-cli current claude

# Health check
ccswitch-cli health claude

# AI mode (XML output)
ccswitch-cli --ai list claude
ccswitch-cli --ai switch claude --provider openrouter --dry-run

# Generate shell completions
ccswitch-cli completions bash > /usr/local/etc/bash_completion.d/ccswitch-cli
ccswitch-cli completions zsh > ~/.zsh/completions/_ccswitch-cli
```

## AI Mode Output Example

```xml
<ccswitch command="list" app_type="claude">
  <providers>
    <provider id="anthropic" name="Anthropic" is_current="true"/>
    <provider id="openrouter" name="OpenRouter" category="aggregator" is_current="false"/>
  </providers>
  <skill_install_hint>npx -g skills add https://github.com/FrankieeW/agent-skills</skill_install_hint>
</ccswitch>
```

## Tech Stack

| Component | Choice |
|-----------|--------|
| CLI Framework | clap 4.5 |
| Database | rusqlite 0.32 |
| Serialization | serde, quick-xml |
| Tables | tabled 0.16 |

## File Structure

```
ccswitch-cli/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── list.rs
│   │   ├── switch.rs
│   │   ├── current.rs
│   │   └── health.rs
│   ├── db/
│   │   ├── mod.rs
│   │   └── models.rs
│   └── formatter/
│       ├── mod.rs
│       ├── human.rs
│       └── ai.rs
└── docs/
    └── PLAN.md
```

## Installation

### Homebrew

```bash
brew tap FrankieeW/homebrew-tap
brew install ccswitch-cli
```

### Manual

Download binary from GitHub Releases and add to PATH.

## Skill Integration

Install skill for AI agents:

```bash
npx -g skills add https://github.com/FrankieeW/agent-skills
```

## Database Schema (Reference)

CC Switch uses SQLite at `~/.cc-switch/cc-switch.db`:

- `providers` - Provider configurations (PK: `(id, app_type)`)
- `provider_health` - Health status (PK: `(provider_id, app_type)`)
- `settings` - Key-value store

### Recommended Indexes

The following indexes improve query performance for the CLI's workload:

```sql
-- Supports list query (ORDER BY sort_index) and health JOIN
CREATE INDEX IF NOT EXISTS idx_providers_app_type
  ON providers(app_type, sort_index);

-- Supports current-provider lookup
CREATE INDEX IF NOT EXISTS idx_providers_app_current
  ON providers(app_type, is_current);

-- Supports health lookup by provider
CREATE INDEX IF NOT EXISTS idx_provider_health_app
  ON provider_health(app_type, provider_id);
```

These are advisory — the CLI works without them but benefits from them on large provider tables.

## TODO Checklist

### Phase 1: Core CLI
- [x] Project initialization
- [x] Database read layer
- [x] List command
- [x] Current command
- [x] Switch command (read-write, atomic transaction)
- [x] Health command
- [x] Human output formatter
- [x] AI (XML) output formatter (well-formed, escaped)

### Phase 2: Completions
- [x] Bash completions
- [x] Elvish completions
- [x] Fish completions
- [x] PowerShell completions
- [x] Zsh completions

### Phase 3: Distribution
- [ ] Create GitHub repo
- [ ] Add README
- [ ] Create Homebrew formula
- [ ] GitHub Actions release workflow

### Phase 4: Skill
- [ ] Add ccswitch skill to agent-skills repo
- [ ] Test skill integration

### Phase 5: Enhancements
- [x] Write support for switch command
- [ ] Import/export configurations
- [ ] Interactive fuzzy search mode
- [ ] Config diff view
