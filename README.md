# ccswitch-cli

CLI for managing CC Switch AI provider configurations. Switch between providers for Claude Code, OpenCode, and other AI coding tools from the command line.

## Installation

### Homebrew

```bash
brew tap FrankieeW/homebrew-tap
brew install ccswitch-cli
```

### Build from Source

```bash
cargo install --path .
```

## Usage

### List Providers

```bash
ccswitch-cli list claude
ccswitch-cli list opencode
```

### Switch Provider

```bash
# Preview changes
ccswitch-cli switch claude --provider openrouter --dry-run

# Apply switch
ccswitch-cli switch claude --provider openrouter
```

### Show Current Provider

```bash
ccswitch-cli current claude
```

### Check Health

```bash
ccswitch-cli health claude
```

## AI Mode

For AI agents, use XML output mode for better parsing:

```bash
ccswitch-cli --ai list claude
ccswitch-cli --ai switch claude --provider openrouter --dry-run
```

## Shell Completions

### Bash

```bash
# Generate and install
ccswitch-cli completions bash > /usr/local/etc/bash_completion.d/ccswitch-cli
# Or for Homebrew-installed bash on macOS:
ccswitch-cli completions bash > $(brew --prefix)/etc/bash_completion.d/ccswitch-cli
```

### Zsh

```bash
# Generate and install
ccswitch-cli completions zsh > ~/.zsh/completions/_ccswitch-cli
```

Add to your `~/.zshrc`:
```zsh
fpath=(~/.zsh/completions $fpath)
autoload -U compinit && compinit
```

### Fish

```bash
ccswitch-cli completions fish > ~/.config/fish/completions/ccswitch-cli.fish
```

## Supported Apps

| App | Description |
|-----|-------------|
| `claude` | Claude Code (Anthropic) |
| `opencode` | OpenCode |
| `openclaw` | OpenClaw |
| `codex` | Codex |
| `gemini` | Gemini |

## License

MIT
