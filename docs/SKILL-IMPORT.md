# FGP Skill Import System

Import skills from agent-specific formats into canonical FGP `skill.yaml` format with automatic quality assessment and sync tracking.

## Quick Start

```bash
# Import from Claude Code SKILL.md
fgp skill import ./SKILL.md --output ./my-skill/

# Import with daemon registry enrichment
fgp skill import ./SKILL.md --enrich --output ./my-skill/

# Preview what would be imported (dry run)
fgp skill import ./SKILL.md --dry-run

# Import from specific format
fgp skill import ./rules.txt --format cursor
```

## Supported Formats

| Format | File Pattern | Fidelity | Notes |
|--------|--------------|----------|-------|
| Claude Code | `SKILL.md` | ~80% | YAML frontmatter + markdown |
| Cursor | `.cursorrules` | ~50% | Pure markdown, inferred daemons |
| Windsurf | `*.windsurf.md` | ~70% | Similar to Claude Code |
| Zed | `*.rules` | ~40% | Context-only format |
| Gemini | `gemini-extension.json` | ~75% | JSON manifest |
| Codex | `*.codex.json` | ~25% | Minimal tool list |
| MCP | `*.mcp.json` | ~30% | Tool definitions only |
| Aider | `CONVENTIONS.md` | ~35% | Style preferences |

## Output Structure

After import, you'll have:

```
my-skill/
├── skill.yaml           # Canonical FGP skill manifest
├── instructions/
│   ├── core.md          # Main instructions (extracted)
│   └── claude-code.md   # Original source (preserved)
├── workflows/
│   └── .gitkeep         # Placeholder for workflows
├── .sync.json           # Sync tracking metadata
└── IMPORT_REPORT.md     # Quality assessment & recommendations
```

## Command Options

```
fgp skill import <path> [options]

Arguments:
  <path>              Path to skill file (SKILL.md, .cursorrules, etc.)

Options:
  -f, --format <fmt>  Source format (auto-detected if not specified)
                      Values: claude-code, cursor, codex, mcp, zed, windsurf, gemini, aider
  -o, --output <dir>  Output directory (default: ./<skill-name>/)
  --dry-run           Preview import without writing files
  --enrich            Enable daemon registry enrichment
  -h, --help          Print help
```

## Quality Grades

The import system assigns a quality grade (A-F) based on how much data was recovered:

| Grade | Score | Meaning |
|-------|-------|---------|
| 🟢 A | 90-100% | Production ready - minimal review needed |
| 🔵 B | 80-89% | Good quality - minor issues to address |
| 🟡 C | 70-79% | Usable - needs review |
| 🟠 D | 60-69% | Incomplete - significant work needed |
| 🔴 F | <60% | Major issues - use as template only |

### Score Breakdown

| Category | Weight | What's Evaluated |
|----------|--------|------------------|
| Metadata | 25% | name, version, description, author, license |
| Daemons | 30% | daemon names, methods, verification |
| Instructions | 25% | content quality, length, code examples |
| Triggers | 10% | keywords, patterns, commands |
| Config/Auth | 10% | auth requirements, platform support |

## Daemon Registry Enrichment

Use `--enrich` to verify daemons and add metadata from FGP daemon manifests:

```bash
fgp skill import ./SKILL.md --enrich
```

Enrichment adds:
- **Verified daemons** - Confirms daemon names exist in registry
- **Method descriptions** - Full descriptions from manifest
- **Auth requirements** - OAuth scopes, API keys needed
- **Platform support** - darwin, linux, windows compatibility

Example enriched output:
```
→ Loading daemon registry...
  ✓ Loaded 9 daemons: [gmail, calendar, github, browser, ...]
  ✓ Verified daemons: [gmail]
  ! Auth required: [gmail]
```

## Sync Tracking

The import system tracks sync state to detect when source or canonical skill changes:

### Sync States

| Status | Emoji | Meaning |
|--------|-------|---------|
| In Sync | ✅ | Source matches canonical - no action needed |
| Source Newer | ⬇️ | Source updated - re-import recommended |
| Canonical Newer | ⬆️ | Canonical updated - re-export recommended |
| Diverged | ⚠️ | Both changed - manual merge required |
| Unknown | ❓ | First import - no sync history |

### How It Works

1. On first import, a fingerprint is generated and saved to `.sync.json`
2. On subsequent imports, the new fingerprint is compared to the stored one
3. If hashes differ, the system recommends appropriate action

### Sync Metadata

The `.sync.json` file contains:
```json
{
  "source_path": "/path/to/SKILL.md",
  "source_format": "claude-code",
  "fingerprint": {
    "combined_hash": 16232158107465268902,
    "timestamp": "2026-01-15T16:21:01.742094+00:00"
  },
  "last_sync": "2026-01-15T16:21:01.742094+00:00",
  "direction": "import"
}
```

## Import Report

Every import generates `IMPORT_REPORT.md` with:

1. **Field Recovery Summary** - Table of extracted fields with confidence
2. **Registry Enrichment** - Verified daemons, auth, methods (if --enrich)
3. **Quality Assessment** - Score breakdown, issues, recommendations
4. **Sync Tracking** - Current status, fingerprint, suggested commands
5. **Unrecoverable Data** - What must be manually added

### Example Report Header

```markdown
# Import Report: gmail

**Source:** /path/to/SKILL.md (Claude Code format)
**Imported:** 2026-01-15T16:21:01.730111+00:00
**Quality Grade:** 🟡 C - Usable - Needs Review (74%)
**Sync Status:** ✅ In sync - no changes
**Enriched:** Yes (daemon registry lookup)
```

## Confidence Levels

Each extracted field has a confidence level:

| Level | Symbol | Meaning |
|-------|--------|---------|
| High | ✓ | Directly from structured data |
| Medium | ⚠ | Inferred from patterns |
| Low | ? | Guessed or placeholder |
| Unknown | ✗ | Could not determine |

## Best Practices

### 1. Always Use `--enrich`

Enrichment significantly improves quality by verifying daemons and adding metadata:

```bash
fgp skill import ./SKILL.md --enrich --output ./my-skill/
```

### 2. Review the Import Report

Check `IMPORT_REPORT.md` for:
- Fields marked with ❌ or ⚠️ that need attention
- Recommendations section for prioritized fixes
- Format limitations that explain missing data

### 3. Fix High-Priority Issues First

Issues are prioritized:
- 🚨 **Critical** - Must fix before using
- ⚠️ **High** - Should fix soon
- 📝 **Medium** - Recommended improvement
- 💡 **Low** - Nice to have

### 4. Keep Source Files

The original source is preserved in `instructions/<format>.md`. This allows:
- Re-importing when source updates
- Comparing canonical vs agent-specific versions
- Round-trip export back to original format

### 5. Track Sync State

Don't delete `.sync.json` - it enables change detection for future imports.

## Troubleshooting

### "Could not detect format"

Specify the format explicitly:
```bash
fgp skill import ./my-rules.txt --format cursor
```

### Low Quality Grade

1. Run with `--enrich` to verify daemons
2. Check Format Limitations in report
3. Manually add missing fields to `skill.yaml`

### "Unknown daemons" Warning

The daemon isn't in the FGP registry. Either:
- It's a custom daemon (add manifest to `~/Projects/fgp/<daemon>/manifest.json`)
- The daemon name is misspelled in the source
- The source uses a different naming convention

### Sync Shows "Source Newer" But Nothing Changed

Hash collision is extremely rare. If you're certain nothing changed:
```bash
# Re-import to reset sync state
fgp skill import ./SKILL.md --output ./my-skill/
```

## Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Source File    │────▶│  Format Parser   │────▶│  Imported Skill │
│  (SKILL.md)     │     │  (per-format)    │     │  (UIR)          │
└─────────────────┘     └──────────────────┘     └────────┬────────┘
                                                          │
                        ┌──────────────────┐              │
                        │ Daemon Registry  │◀─────────────┤ --enrich
                        │ (manifests)      │              │
                        └────────┬─────────┘              │
                                 │                        │
                                 ▼                        ▼
                        ┌──────────────────┐     ┌─────────────────┐
                        │ Enrichment Data  │────▶│ Quality Analysis│
                        │ (auth, methods)  │     │ (scoring)       │
                        └──────────────────┘     └────────┬────────┘
                                                          │
                                                          ▼
                        ┌──────────────────┐     ┌─────────────────┐
                        │ Sync Analysis    │◀────│ Output Files    │
                        │ (.sync.json)     │     │ (skill.yaml)    │
                        └──────────────────┘     └─────────────────┘
```

## Related Commands

```bash
# Export skill to agent format
fgp skill export claude-code ./my-skill/ --output ./

# Validate skill manifest
fgp skill validate ./my-skill/

# List installed skills
fgp skill list

# Search for skills
fgp skill search "gmail"
```
