# Migrating from MCP to FGP

This guide walks you through migrating MCP (Model Context Protocol) servers and configurations to FGP (Fast Gateway Protocol) daemons.

## Why Migrate?

| Aspect | MCP | FGP |
|--------|-----|-----|
| **Startup time** | ~2,300ms (cold start) | ~10-50ms (warm daemon) |
| **Architecture** | Stdio process per call | Persistent UNIX socket daemon |
| **Memory** | New process each time | Single process, ~10MB |
| **Concurrency** | Limited by process spawn | Native async/parallel |
| **Tool calls** | Sequential bottleneck | 10-100x faster |

## Migration Paths

### Path 1: Import Existing MCP Config

If you have an MCP configuration file, import it directly:

```bash
# Import MCP tool definitions
fgp skill import ./tools.mcp.json --output ./my-skill/

# Preview what would be imported
fgp skill import ./tools.mcp.json --dry-run
```

**Note:** MCP format has low fidelity (~25%) because it only contains tool schemas. You'll need to manually add:
- Instructions (how to use the tools)
- Triggers (when the skill should activate)
- Workflows (multi-step operations)

### Path 2: Convert MCP Server to FGP Daemon

For full performance benefits, convert your MCP server to an FGP daemon.

#### Before (MCP Server - TypeScript)

```typescript
// mcp-server.ts
import { Server } from '@modelcontextprotocol/sdk/server';

const server = new Server({ name: 'my-mcp-server' });

server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [
    {
      name: 'my_tool',
      description: 'Does something',
      inputSchema: { type: 'object', properties: { input: { type: 'string' } } }
    }
  ]
}));

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  if (request.params.name === 'my_tool') {
    return { content: [{ type: 'text', text: 'result' }] };
  }
});

server.connect(new StdioServerTransport());
```

#### After (FGP Daemon - Rust)

```rust
// src/main.rs
use fgp_daemon::{FgpServer, FgpService, Result};
use serde_json::{json, Value};
use std::collections::HashMap;

struct MyService;

impl FgpService for MyService {
    fn name(&self) -> &str { "my-service" }
    fn version(&self) -> &str { "1.0.0" }

    fn dispatch(&self, method: &str, params: HashMap<String, Value>) -> Result<Value> {
        match method {
            "my-service.my_tool" | "my_tool" => {
                let input = params.get("input")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                Ok(json!({ "result": format!("processed: {}", input) }))
            }
            _ => bail!("Unknown method: {}", method),
        }
    }
}

fn main() -> Result<()> {
    let server = FgpServer::new(
        MyService,
        "~/.fgp/services/my-service/daemon.sock"
    )?;
    server.serve()
}
```

#### After (FGP Daemon - Python)

```python
# daemon.py
from fgp_daemon import FgpServer, FgpService

class MyService(FgpService):
    def name(self) -> str:
        return "my-service"

    def version(self) -> str:
        return "1.0.0"

    def dispatch(self, method: str, params: dict) -> dict:
        if method in ("my-service.my_tool", "my_tool"):
            input_val = params.get("input", "")
            return {"result": f"processed: {input_val}"}
        raise ValueError(f"Unknown method: {method}")

if __name__ == "__main__":
    server = FgpServer(MyService())
    server.serve()
```

### Path 3: Hybrid Approach

Keep MCP for complex tools while migrating performance-critical ones to FGP:

1. Import MCP config to get tool inventory
2. Identify slow/frequent tools from usage patterns
3. Convert those tools to FGP daemon methods
4. Update skill to use FGP daemon for fast paths

## Step-by-Step Migration

### Step 1: Inventory Your MCP Tools

```bash
# List MCP tools in your configuration
cat ~/.claude.json | jq '.mcpServers | keys'
```

### Step 2: Import MCP Configuration

```bash
# Create skill from MCP config
fgp skill import ~/.config/mcp/tools.json --output ./migrated-skill/
```

### Step 3: Review Import Report

Check `IMPORT_REPORT.md` for:
- Tools that were detected
- Missing information that needs manual addition
- Recommendations for improvement

### Step 4: Create FGP Daemon

```bash
# Generate daemon skeleton from skill
fgp daemon init ./migrated-skill/ --output ./my-daemon/
```

### Step 5: Implement Methods

Copy logic from MCP handlers to FGP dispatch methods:

| MCP Pattern | FGP Pattern |
|-------------|-------------|
| `server.setRequestHandler(CallToolRequestSchema, ...)` | `impl FgpService { fn dispatch(...) }` |
| `request.params.name === 'tool'` | `match method { "daemon.tool" => ... }` |
| `{ content: [{ type: 'text', text: '...' }] }` | `Ok(json!({ ... }))` |

### Step 6: Test Performance

```bash
# Benchmark MCP vs FGP
hyperfine \
  'echo "{}" | mcp-tool call my_tool' \
  'fgp call my-service.my_tool'
```

### Step 7: Update Agent Configuration

Replace MCP server reference with FGP skill:

```yaml
# Before (MCP in claude.json)
mcpServers:
  my-server:
    command: node
    args: ["./mcp-server.js"]

# After (FGP skill)
# Skills are auto-detected from ~/.claude/skills/my-skill/SKILL.md
```

## Tool Name Mapping

MCP and FGP use different naming conventions:

| MCP Format | FGP Format |
|------------|------------|
| `mcp__github__list_repos` | `github.list_repos` |
| `mcp__slack__send_message` | `slack.send_message` |
| `my_tool` | `my-service.my_tool` |

The import system automatically converts MCP tool names to FGP format.

## Common Migration Issues

### Issue: "Unknown daemon" after import

MCP tools use server names, not daemon names. Map them manually:

```yaml
# In skill.yaml, update daemon references
daemons:
  - name: github  # Not "mcp__github"
    methods:
      - list_repos
      - create_issue
```

### Issue: Complex input schemas

MCP uses JSON Schema for inputs. FGP uses simpler parameter maps:

```rust
// MCP (complex schema)
inputSchema: {
  type: 'object',
  properties: {
    filters: {
      type: 'object',
      properties: {
        status: { enum: ['open', 'closed'] }
      }
    }
  }
}

// FGP (simple params)
fn dispatch(&self, method: &str, params: HashMap<String, Value>) -> Result<Value> {
    let filters = params.get("filters").and_then(|v| v.as_object());
    let status = filters.and_then(|f| f.get("status")).and_then(|s| s.as_str());
    // ...
}
```

### Issue: Streaming responses

MCP supports streaming via SSE. FGP uses NDJSON for streaming:

```rust
// FGP streaming (if needed)
fn dispatch_streaming(&self, method: &str, params: HashMap<String, Value>)
    -> impl Stream<Item = Result<Value>> {
    // Return async stream of JSON values
}
```

## Performance Comparison

After migration, expect these improvements:

| Operation | MCP | FGP | Speedup |
|-----------|-----|-----|---------|
| First call | ~2,300ms | ~50ms | 46x |
| Subsequent calls | ~2,300ms | ~10ms | 230x |
| Parallel calls | Sequential | Concurrent | N/A |
| Memory per call | ~50MB | ~0MB | ∞ |

## Further Reading

- [FGP Protocol Specification](https://github.com/fast-gateway-protocol/protocol)
- [Building FGP Daemons](https://github.com/fast-gateway-protocol/daemon)
- [Skill Import Reference](./SKILL-IMPORT.md)
- [Import Format Details](./IMPORT-FORMATS.md)
