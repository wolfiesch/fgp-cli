# Skill Import Format Reference

Detailed documentation for each supported import format.

## Claude Code (SKILL.md)

**Fidelity: ~80%** | **Pattern: `SKILL.md`**

The highest fidelity format. Uses YAML frontmatter for structured metadata.

### Structure

```markdown
---
name: my-skill
description: A helpful skill
version: 1.0.0
tools:
  - daemon: gmail
    methods:
      - inbox
      - send
triggers:
  keywords:
    - email
    - gmail
---

# My Skill

Instructions content here...

## Usage

More content...
```

### What's Extracted

| Field | Source | Confidence |
|-------|--------|------------|
| name | Frontmatter `name` | High |
| description | Frontmatter `description` | High |
| version | Frontmatter `version` | High |
| daemons | Frontmatter `tools` | High |
| methods | Frontmatter `tools[].methods` | High |
| triggers | Frontmatter `triggers` | High |
| instructions | Markdown body | High |

### Limitations

- Workflows not included in export format
- Config options not recoverable
- Some auth requirements may need enrichment

---

## Cursor (.cursorrules)

**Fidelity: ~50%** | **Pattern: `.cursorrules`, `*.cursorrules`**

Pure markdown format with no structured metadata. Daemons/methods must be inferred.

### Structure

```markdown
# Project Rules

You are helping with a project that uses Gmail and Calendar.

## Available Tools

- gmail.inbox - List emails
- gmail.send - Send email
- calendar.list - List events

## Guidelines

Always be helpful...
```

### What's Extracted

| Field | Source | Confidence |
|-------|--------|------------|
| name | First H1 header or filename | Medium |
| description | First paragraph after H1 | Medium |
| version | Default `1.0.0` | Low |
| daemons | Pattern matching `daemon.method` | Medium |
| methods | Pattern matching | Medium |
| triggers | Extracted from content | Low |
| instructions | Full content | High |

### Detection Patterns

Methods are detected via regex:
- `daemon.method` format (e.g., `gmail.send`)
- `mcp__server__tool` format
- Bullet lists under "Tools" or "Available" sections

### Limitations

- No version or author information
- Daemon/method info inferred from text
- No structured trigger configuration
- Pure markdown has low fidelity

---

## Windsurf (*.windsurf.md)

**Fidelity: ~70%** | **Pattern: `*.windsurf.md`**

Similar to Claude Code with YAML frontmatter, but may have different field names.

### Structure

```markdown
---
name: my-cascade
description: Cascade rules for project
capabilities:
  - name: email
    tools: [gmail.send, gmail.read]
---

# Cascade Rules

Instructions here...
```

### What's Extracted

| Field | Source | Confidence |
|-------|--------|------------|
| name | Frontmatter `name` | High |
| description | Frontmatter `description` | High |
| version | Frontmatter `version` or default | Medium |
| daemons | Frontmatter `capabilities` | Medium |
| instructions | Markdown body | High |

### Limitations

- Similar to Claude Code limitations
- Capabilities may not map directly to FGP daemons

---

## Zed (*.rules)

**Fidelity: ~40%** | **Pattern: `*.rules`**

Context-only format for Zed's AI assistant.

### Structure

```
You are an AI assistant helping with this project.

Available tools:
- Search files
- Read documentation
- Execute commands

Guidelines:
- Always explain your reasoning
- Use TypeScript
```

### What's Extracted

| Field | Source | Confidence |
|-------|--------|------------|
| name | Filename (without .rules) | Medium |
| description | First sentence | Low |
| version | Default `1.0.0` | Low |
| daemons | Pattern matching | Low |
| instructions | Full content | High |

### Limitations

- No structured metadata
- Rules may not map to FGP concepts
- Context-only format

---

## Gemini (gemini-extension.json)

**Fidelity: ~75%** | **Pattern: `gemini-extension.json`**

JSON manifest with structured capability definitions.

### Structure

```json
{
  "name": "Calendar Assistant",
  "version": "2.1.0",
  "description": "Manage Google Calendar",
  "capabilities": [
    {
      "name": "list",
      "description": "List calendar events"
    },
    {
      "name": "create",
      "description": "Create new event"
    }
  ],
  "triggers": {
    "keywords": ["calendar", "schedule", "meeting"]
  }
}
```

### What's Extracted

| Field | Source | Confidence |
|-------|--------|------------|
| name | `name` field | High |
| description | `description` field | High |
| version | `version` field | High |
| daemons | Inferred from capabilities | Medium |
| methods | `capabilities[].name` | Medium |
| triggers | `triggers` object | High |
| instructions | `instructions` field | High |

### Limitations

- Capabilities may not map directly to FGP methods
- Extension config not all recoverable

---

## Codex (*.codex.json)

**Fidelity: ~25%** | **Pattern: `*.codex.json`, `config.codex.json`**

Minimal JSON schema with tool list only.

### Structure

```json
{
  "name": "file-manager",
  "description": "File operations helper",
  "tools": ["fs.read", "fs.write", "fs.list"],
  "instructions": "Help users manage files safely."
}
```

### What's Extracted

| Field | Source | Confidence |
|-------|--------|------------|
| name | `name` field | High |
| description | `description` field | High |
| version | Default `1.0.0` | Low |
| daemons | Inferred from `tools` | Low |
| methods | Parsed from `tools` | Low |
| instructions | `instructions` field | Medium |

### Tool Parsing

Tools are parsed as `daemon.method`:
- `fs.read` → daemon: `fs`, method: `read`
- `github.issues` → daemon: `github`, method: `issues`

### Limitations

- Minimal schema format
- No detailed instructions
- Tool list only, no method parameters
- No trigger or auth configuration

---

## MCP (*.mcp.json)

**Fidelity: ~30%** | **Pattern: `*.mcp.json`, `tools.mcp.json`**

MCP tool schema format.

### Structure

```json
{
  "name": "github-tools",
  "description": "GitHub repository management",
  "tools": [
    {
      "name": "mcp__github__list_repos",
      "description": "List user repositories"
    },
    {
      "name": "mcp__github__create_issue",
      "description": "Create a new issue"
    }
  ]
}
```

### What's Extracted

| Field | Source | Confidence |
|-------|--------|------------|
| name | `name` field | High |
| description | `description` field | High |
| version | Default `1.0.0` | Low |
| daemons | Extracted from tool names | Medium |
| methods | Parsed from MCP format | Medium |
| instructions | Generated from descriptions | Low |

### Tool Name Parsing

MCP tool names are parsed:
- `mcp__github__list_repos` → daemon: `github`, method: `list_repos`
- `github.commit` → daemon: `github`, method: `commit`

### Limitations

- Tool definitions only
- No workflow or trigger information
- Method names may need translation

---

## Aider (CONVENTIONS.md)

**Fidelity: ~35%** | **Pattern: `CONVENTIONS.md`, `*.CONVENTIONS.md`**

Project conventions and style preferences.

### Structure

```markdown
# Project Conventions

## Code Style

- Use TypeScript strict mode
- Prefer functional components
- Use ESLint with recommended rules

## Architecture

- Follow hexagonal architecture
- Keep business logic in services
- Use dependency injection

## Testing

- Write unit tests for all services
- Use Jest for testing
```

### What's Extracted

| Field | Source | Confidence |
|-------|--------|------------|
| name | Filename or "conventions" | Low |
| description | First paragraph | Low |
| version | Default `1.0.0` | Low |
| daemons | Pattern matching | Low |
| instructions | Full content | High |

### Limitations

- Conventions format is minimal
- No tool/daemon definitions typically
- Style preferences only
- Best used as supplementary instructions

---

## Format Detection

The import system auto-detects format by filename:

| Pattern | Detected Format |
|---------|-----------------|
| `SKILL.md` | Claude Code |
| `*.cursorrules`, `.cursorrules` | Cursor |
| `*.windsurf.md` | Windsurf |
| `*.rules` | Zed |
| `gemini-extension.json` | Gemini |
| `*.codex.json` | Codex |
| `*.mcp.json` | MCP |
| `CONVENTIONS.md`, `*.CONVENTIONS.md` | Aider |

Override with `--format`:
```bash
fgp skill import ./custom-file.txt --format cursor
```

---

## Comparison Matrix

| Feature | Claude | Cursor | Windsurf | Zed | Gemini | Codex | MCP | Aider |
|---------|--------|--------|----------|-----|--------|-------|-----|-------|
| Structured metadata | ✅ | ❌ | ✅ | ❌ | ✅ | ⚠️ | ⚠️ | ❌ |
| Version info | ✅ | ❌ | ⚠️ | ❌ | ✅ | ❌ | ❌ | ❌ |
| Daemon definitions | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ❌ |
| Method details | ✅ | ⚠️ | ⚠️ | ❌ | ⚠️ | ⚠️ | ✅ | ❌ |
| Triggers | ✅ | ⚠️ | ⚠️ | ❌ | ✅ | ❌ | ❌ | ❌ |
| Instructions | ✅ | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ✅ |
| Author info | ⚠️ | ❌ | ⚠️ | ❌ | ⚠️ | ❌ | ❌ | ❌ |

Legend: ✅ Good support | ⚠️ Partial/inferred | ❌ Not available

---

## Adding New Formats

To add support for a new agent format:

1. Add format variant to `ImportFormat` enum in `skill_import.rs`
2. Update `ImportFormat::detect()` with filename patterns
3. Implement `parse_<format>()` function
4. Add to match statement in `import_skill()`
5. Add format to `get_format_limitations()`
6. Document in this file
