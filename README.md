# fgp

[![CI](https://github.com/fast-gateway-protocol/cli/actions/workflows/ci.yml/badge.svg)](https://github.com/fast-gateway-protocol/cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/fgp.svg)](https://crates.io/crates/fgp)

Command-line interface for [Fast Gateway Protocol (FGP)](https://github.com/fast-gateway-protocol) daemons.

FGP is the universal package manager for AI agents. One command installs capabilities across Claude Code, Cursor, Windsurf, and other AI coding assistants.

## Installation

```bash
cargo install fgp
```

Or build from source:

```bash
git clone https://github.com/fast-gateway-protocol/cli
cd fgp-cli
cargo install --path .
```

## Quick Start

```bash
# Detect which AI agents you have installed
fgp agents

# Check status of running daemons
fgp status

# Install a package (installs daemon + skills for all detected agents)
fgp install ./my-package/

# Start a daemon
fgp start gmail

# Call a method
fgp call gmail.list -p '{"limit": 10}'

# Stop a daemon
fgp stop gmail
```

## Commands

| Command | Description |
|---------|-------------|
| `fgp agents` | Detect installed AI agents (Claude Code, Cursor, Windsurf, etc.) |
| `fgp status` | Show status of all running FGP daemons |
| `fgp start <service>` | Start a daemon service |
| `fgp stop <service>` | Stop a running daemon |
| `fgp call <method>` | Call a method on a daemon |
| `fgp methods <service>` | List available methods for a service |
| `fgp health <service>` | Check health of a specific service |
| `fgp install <path>` | Install a package from local path |
| `fgp skill import <path>` | Import skills from other agent formats |
| `fgp skill export <format>` | Export skill to agent-specific format |

## Skill Import

Import existing skills from Claude Code, Cursor, Windsurf, Gemini, and other AI agents into FGP's canonical `skill.yaml` format.

```bash
# Import from Claude Code SKILL.md
fgp skill import ./SKILL.md --output ./my-skill/

# Import from Cursor rules
fgp skill import ./.cursorrules --output ./my-skill/

# Import from Gemini extension
fgp skill import ./gemini-extension.json --output ./my-skill/

# Preview import without writing files (dry run)
fgp skill import ./SKILL.md --dry-run

# Import with daemon registry enrichment (recommended)
fgp skill import ./SKILL.md --enrich --output ./my-skill/
```

### Supported Formats

| Format | Pattern | Quality |
|--------|---------|---------|
| Claude Code | `SKILL.md` | 🔵 B (87%) |
| Gemini | `gemini-extension.json` | 🔵 B (88%) |
| Windsurf | `*.windsurf.md` | 🔵 B (87%) |
| Cursor | `.cursorrules` | 🟡 C (76%) |
| Zed | `*.rules` | 🟡 C (75%) |
| Codex | `*.codex.json` | 🟡 C (75%) |
| Aider | `CONVENTIONS.md` | 🟡 C (74%) |

See [docs/SKILL-IMPORT.md](docs/SKILL-IMPORT.md) for full documentation and [docs/IMPORT-FORMATS.md](docs/IMPORT-FORMATS.md) for format-specific details.

## Agent Detection

FGP automatically detects these AI agents:

- **Claude Code** (`~/.claude/skills/`) - SKILL.md files
- **Cursor** (`~/.cursor/`) - .mdc rules
- **Windsurf** (`~/.windsurf/`) - Workflow files
- **Continue** (`~/.continue/`) - YAML config
- **Cline** - MCP configuration

When you install an FGP package, skill files are automatically created for all detected agents.

## Example Output

```bash
$ fgp agents
Detecting installed AI agents...

  ✓ Claude Code
    Path: ~/.claude/skills
    Format: SKILL.md files

  ✓ Cursor
    Path: ~/.cursor
    Format: .mdc rules

FGP packages will automatically install skill files for detected agents.
```

```bash
$ fgp status
FGP Services

+----------+-----------+---------+--------+
| Service  | Status    | Version | Uptime |
+----------+-----------+---------+--------+
| gmail    | ● running | 1.0.0   | 2h 15m |
| imessage | ● running | 1.0.0   | 5d 3h  |
| github   | ○ stopped | -       | -      |
+----------+-----------+---------+--------+
```

## Related Projects

- [protocol](https://github.com/fast-gateway-protocol/protocol) - FGP Protocol Specification
- [daemon](https://github.com/fast-gateway-protocol/daemon) - Rust SDK for building FGP daemons

## License

MIT
