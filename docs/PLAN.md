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
  - [x] AI mode (XML output)

### Pending

- [ ] GitHub repository creation
- [ ] Shell completions (zsh, bash)
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

- `providers` - Provider configurations
- `provider_health` - Health status
- `settings` - Key-value store

## TODO Checklist

### Phase 1: Core CLI
- [x] Project initialization
- [x] Database read layer
- [x] List command
- [x] Current command
- [x] Switch command (read-only)
- [x] Health command
- [x] Human output formatter
- [x] AI (XML) output formatter

### Phase 2: Completions
- [ ] Zsh completions
- [ ] Bash completions

### Phase 3: Distribution
- [ ] Create GitHub repo
- [ ] Add README
- [ ] Create Homebrew formula
- [ ] GitHub Actions release workflow

### Phase 4: Skill
- [ ] Add ccswitch skill to agent-skills repo
- [ ] Test skill integration

### Phase 5: Enhancements
- [ ] Write support for switch command
- [ ] Import/export configurations
- [ ] Interactive fuzzy search mode
- [ ] Config diff view
