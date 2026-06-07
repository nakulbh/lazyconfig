# lazyconfig — Implementation Plan

## Project Overview

A Rust/GPUI desktop app to manage AI coding agent configuration — skills, subagents, MCP servers, rules, hooks, instructions, and settings — across 21+ agent platforms from a single UI with both global and per-project views.

**Differentiator:** No existing tool combines skill inventory + subagent management + MCP management + per-project view + cross-agent editing in one place.

**Status:** Phase 0 (scaffold started)

---

## Target Agent Platforms

### Phase 1 Targets (4 agents)
| Platform | Global Config Dir | Project Config Dir | Skills | Subagents | MCP |
|---|---|---|---|---|---|
| Claude Code | `~/.claude/` | `.claude/` | SKILL.md (YAML) | `.md` (YAML frontmatter) | `.mcp.json` |
| OpenCode | `~/.config/opencode/` | `.opencode/` | SKILL.md (YAML) | `.md` (YAML frontmatter) or inline JSON | `opencode.json` `mcp` key |
| Codex CLI | `~/.codex/` | `.codex/` | SKILL.md (YAML) | `.toml` | `config.toml` `[mcp_servers]` |
| GitHub Copilot | `~/.copilot/` | `.github/` | SKILL.md (YAML) | `.md` (YAML frontmatter) | `mcp.json` |

### Phase 6+ Targets (17 additional agents)
Cursor, Windsurf, Gemini CLI, Cline, AMP, Antigravity, ClawdBot, Droid, Goose, Kilo, Kiro CLI, Nous Research, Roo Code, Trae, VS Code, Zed, Cursor

All 21 read SKILL.md via the Agent Skills open standard.

---

## Config Artifact Types

| Artifact | Claude Code | OpenCode | Codex | Copilot |
|---|---|---|---|---|
| **Skills** | `.claude/skills/*/SKILL.md` | `.opencode/skills/*/SKILL.md` + reads Claude's | `.agents/skills/*/SKILL.md` | `.github/skills/*/SKILL.md` |
| **Subagents** | `.claude/agents/*.md` | `agents/*.md` + inline JSON | `.codex/agents/*.toml` | `.github/agents/*.md` |
| **MCP Servers** | `.mcp.json` | `opencode.json` `mcp` key | `config.toml` `[mcp_servers]` | `mcp.json` |
| **Rules** | `.claude/rules/*.md` (path-scoped) | — | — | `.github/instructions/*.instructions.md` |
| **Hooks** | `.claude/hooks/` | Plugins (via JS/TS) | `config.toml` `[hooks]` | `.github/hooks/*.json` |
| **Instructions** | `CLAUDE.md` | `AGENTS.md`, profiles | `AGENTS.md` | `.github/copilot-instructions.md` |
| **Memory** | `~/.claude/projects/<name>/memory/` | — | — | — |
| **Commands** | `.claude/commands/*.md` | `commands/*.md` | — | — |
| **Settings** | `settings.json` | `opencode.json` | `config.toml` | UI-only |
| **Prompt Templates** | — | — | — | `.github/prompts/*.prompt.md` |
| **Themes** | `~/.claude/themes/` | `themes/` | — | — |

### Agent Skills Standard (SKILL.md Format)

YAML frontmatter between `---` delimiters:

| Field | Required | Constraints |
|---|---|---|
| `name` | Yes | 1-64 chars, `[a-z0-9-]+`, must match directory name |
| `description` | Yes | 1-1024 chars, describes what + when to use |
| `license` | No | License name or reference |
| `compatibility` | No | 1-500 chars, environment requirements |
| `metadata` | No | String→string KV map |
| `allowed-tools` | No | Space-separated tool names (experimental) |

---

## Architecture

### Three-Layer Design

```
UI Layer (GPUI Views)
  MainView, Sidebar, SkillList, AgentList, MCPList, DetailPanel,
  EditorPanel, CreateDialog, DiffViewer
    |
Service Layer
  Scanner: walk filesystem, discover all config artifacts
  Parser: parse YAML frontmatter, TOML, JSON per platform
  Writer: serialize + write back in correct format
  ProjectDetector: find git repos with agent config dirs
  Validator: validate against Agent Skills spec
    |
Model Layer
  Skill, Subagent, MCPServer, Rule, Hook, Instruction,
  Project, AgentPlatform enum, Scope enum
```

### Data Models

```rust
enum AgentPlatform {
    ClaudeCode, OpenCode, CodexCli, GitHubCopilot,
    // Phase 6+: Cursor, Windsurf, GeminiCli, Cline, Amp,
    //          Antigravity, ClawdBot, Droid, Goose, Kilo,
    //          KiroCli, NousResearch, RooCode, Trae, VSCode, Zed
}

enum Scope { Global, Project(PathBuf) }

struct Skill {
    name: String,           // from frontmatter, 1-64 chars
    description: String,    // from frontmatter, 1-1024 chars
    license: Option<String>,
    compatibility: Option<String>,
    metadata: HashMap<String, String>,
    allowed_tools: Option<String>,
    body: String,           // markdown body after frontmatter
    path: PathBuf,          // absolute path to SKILL.md
    platform: AgentPlatform,
    scope: Scope,
}

struct Subagent {
    name: String,
    description: String,
    model: Option<String>,
    tools: Vec<String>,
    disallowed_tools: Vec<String>,
    mcp_servers: Vec<String>,
    permission_mode: Option<String>,
    body: String,
    path: PathBuf,
    platform: AgentPlatform,
    scope: Scope,
    format: AgentDefFormat, // MarkdownFrontmatter | Toml | InlineJson
}

struct MCPServer {
    name: String,
    transport: MCPTransport, // Stdio { command, args, env } | HTTP { url, headers }
    enabled: bool,
    platform: AgentPlatform,
    scope: Scope,
}
```

### Component Tree (GPUI Views)

```
Window
├── Sidebar (~250px)
│   ├── "Global" section (all agents, global scope)
│   └── Projects section (auto-detected git repos)
│       └── Project entries (click to switch scope)
└── Content (tabs)
    ├── Skills Tab
    │   ├── FilterBar (search + platform filter + scope filter)
    │   └── SkillList (virtualized, grouped by AgentPlatform)
    │       └── SkillCard (name, desc, platform badge, scope badge)
    ├── Subagents Tab
    │   ├── FilterBar
    │   └── AgentList (grouped by platform)
    │       └── AgentCard (name, desc, tools, model, platform badge)
    ├── MCP Tab (Phase 2.5)
    │   ├── FilterBar
    │   └── MCPList (grouped by platform)
    │       └── MCPCard (name, transport, enabled status, platform badge)
    └── Detail Panel (right side or bottom split)
        ├── Read mode: formatted display of all fields
        └── Edit mode: inline-editable form + markdown body editor
```

### File Structure

```
lazyconfig/
├── Cargo.toml
├── plan.md                  # This file
├── AGENTS.md                # Project instructions for AI agents
├── src/
│   ├── main.rs              # App entry, window creation
│   ├── models/
│   │   ├── mod.rs
│   │   ├── skill.rs         # Skill struct + frontmatter parser
│   │   ├── agent.rs         # Subagent struct + multi-format parsing
│   │   ├── mcp.rs           # MCPServer struct
│   │   ├── rule.rs          # Rule/Hook/Instruction structs
│   │   ├── project.rs       # Project struct
│   │   └── platform.rs      # AgentPlatform enum + path constants
│   ├── services/
│   │   ├── mod.rs
│   │   ├── scanner.rs       # Unified scanner: walk all platform paths
│   │   ├── parser.rs        # YAML frontmatter, TOML, JSON parsing
│   │   ├── writer.rs        # Serialize + write back (preserve formatting)
│   │   ├── validator.rs     # Validate SKILL.md against Agent Skills spec
│   │   └── project_detector.rs  # Find git repos with agent config dirs
│   ├── views/
│   │   ├── mod.rs
│   │   ├── main_view.rs     # Root layout: sidebar + content tabs
│   │   ├── sidebar.rs       # Project list + scope toggles
│   │   ├── skill_list.rs    # Filterable skill inventory
│   │   ├── agent_list.rs    # Filterable subagent inventory
│   │   ├── mcp_list.rs      # MCP server inventory
│   │   ├── detail_panel.rs  # Selected item details (read + edit)
│   │   └── create_dialog.rs # New item creation dialog
│   ├── components/
│   │   ├── mod.rs
│   │   ├── button.rs        # Clickable with hover/active states
│   │   ├── text_input.rs    # Single-line text input
│   │   ├── text_area.rs     # Multi-line markdown editor
│   │   ├── tabs.rs          # Tab bar component
│   │   ├── badge.rs         # Color-coded agent platform badge
│   │   ├── split_view.rs    # Resizable panel divider
│   │   └── filter_bar.rs    # Search + dropdown filter
│   ├── actions.rs           # GPUI action definitions
│   ├── state.rs             # Global app state
│   └── theme.rs             # Colors, spacing, typography
└── assets/
    └── icons/               # Agent platform SVG icons
```

---

## Implementation Phases

### Phase 1: Project Scaffold + Core Widgets (~3-4 days)

**Status:** In Progress

- Initialize Cargo project with GPUI dependency (from `zed-industries/zed` git repo)
- Create `main.rs` with window + root view
- Build minimal custom widget library:
  - `Button` — clickable with hover/active states
  - `TextInput` — single-line, cursor, selection
  - `TextArea` — multi-line, scrollable
  - `Tabs` — tab bar with active state
  - `Badge` — colored pill label for agent platforms
  - `FilterBar` — search input + dropdown
  - `SplitView` — resizable panel divider
- Define colors/theme in `theme.rs`
- Define all actions in `actions.rs`

### Phase 2: Scanner + Parser Layer (~4-5 days)

- Implement `AgentPlatform` enum with hardcoded path constants
- Implement `Scope` enum (Global / Project)
- Implement `Skill` model with `parse_frontmatter()` taking raw text → `Skill`
- Implement `Subagent` model with format-specific parsers:
  - `parse_md_frontmatter()` for Claude/OpenCode/Copilot agents
  - `parse_toml_agent()` for Codex agents
  - `parse_opencode_json_agent()` for inline JSON agents
- Implement `Scanner` service — walk all platform paths, discover all config artifacts
- Implement `ProjectDetector` — find git repos in common locations
- Unit tests for all parsers

### Phase 2.5: MCP Manager (~3-4 days)

- Implement `MCPServer` model — parse JSON/TOML MCP configs
- Implement `MCPList` view — grouped by platform, show transport and status
- Enable/disable servers per agent
- Visual diff of MCP config between agents
- Write operations: add/remove/edit MCP server definitions

### Phase 3: UI Views — Inventory Display (~5-6 days)

- Implement `MainView` — root layout using `SplitView`
- Implement `Sidebar` — project list + scope toggles
- Implement `SkillList` view — virtualized, grouped by platform
- Implement `AgentList` view — same layout pattern
- Implement `DetailPanel` (read mode) — formatted display of all fields

### Phase 4: Write Operations (~4-5 days)

- Implement `Writer` service — serialize models back to file format
- Add edit mode to `DetailPanel` — inline-editable fields
- Implement `CreateDialog` — pick platform/scope, create skill/agent file
- Validation: name constraints, required fields, char limits

### Phase 5: Rules & Instructions Manager (~3-4 days)

- Discover and edit `CLAUDE.md`, `AGENTS.md`, `copilot-instructions.md`
- Path-scoped rules editor (Claude Code `.claude/rules/`, Copilot `.github/instructions/`)
- Preview active rules for a given file path
- Hooks management — view, edit, validate hook scripts

### Phase 6: Marketplace + Cross-Agent (~4-5 days)

- Browse and install skills from `skills.sh` marketplace
- Copy skills between agent platforms
- Conflict detection — warn if same skill name exists at target
- Drag-and-drop in sidebar for quick moves

### Phase 7: Validation & Polish (~3-4 days)

- Validate SKILL.md against Agent Skills spec
- Validate agent definitions per-platform format
- Diff viewer for config changes
- Keyboard shortcuts, empty states, error handling
- File watcher: detect external changes

---

## Key Technical Decisions

| Decision | Rationale |
|---|---|
| **GPUI** | Native GPU rendering, single Rust binary, no web stack. User specified. |
| **Build widgets from scratch** | GPUI has no built-in widgets. Reference: Zed's `crates/ui/` |
| **YAML frontmatter via `serde_yaml`** | Standard Rust YAML lib for `---` delimited frontmatter |
| **TOML via `toml` crate** | For Codex agent definitions |
| **Preserve formatting on write** | Careful serialization to avoid destroying hand-edited files |
| **Virtualized lists via GPUI `list()`** | Performance with 50+ items across platforms |
| **macOS first** | User's platform; Linux/Windows later |
| **Single-window** | Split panes simpler than multi-window |

## Risks & Mitigations

| Risk | Mitigation |
|---|---|
| GPUI breaking changes (pre-1.0) | Pin to specific git commit |
| Config formats evolving rapidly | Forgiving parsers (ignore unknown fields) |
| Complex widgets from scratch | Study Zed's `crates/ui/src/` extensively |
| fmt task | always run `cargo fmt` before pushing |

## External References

- Agent Skills Spec: https://agentskills.io/specification
- Skills Marketplace: https://skills.sh (21+ agents supported)
- GPUI Docs: https://gpui.rs
- GPUI Repo: https://github.com/zed-industries/zed
- Claude Code Skills Docs: https://code.claude.com/docs/en/skills
- OpenCode Config: https://opencode.ai/config.json
- skills-manager (competitor): https://github.com/xingkongliang/skills-manager
