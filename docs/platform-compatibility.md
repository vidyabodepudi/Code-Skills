# Platform Compatibility — CodeHands

## Supported Platforms

| Platform | Adapter File | Model Detection | Enforcement Style | Status |
|---|---|---|---|---|
| **Claude Code** | `adapters/CLAUDE.md` → `CLAUDE.md` | `CLAUDE.md` or `.claude/` presence | Human-partner collaborative | ✅ Full support |
| **Gemini CLI** | `adapters/GEMINI.md` → `GEMINI.md` | `GEMINI.md` or `.gemini/` presence | Structured-checklist | ✅ Full support |
| **Cursor** | `adapters/.cursor-plugin/manifest.json` | `.cursor/` presence | Human-partner collaborative | ✅ Full support |
| **GitHub Copilot** | `adapters/copilot-instructions.md` → `.github/copilot-instructions.md` | `.github/copilot-instructions.md` | Directive-imperative | ✅ Full support |
| **Codex** | `adapters/AGENTS.md` → `AGENTS.md` | `AGENTS.md` + `$CODEX_MODEL` env | Directive-imperative | ✅ Full support |
| **OpenCode** | `adapters/AGENTS.md` → `AGENTS.md` | `AGENTS.md` (no Codex env) | Directive-imperative | ✅ Full support |
| **Windsurf** | `adapters/.windsurfrules` → `.windsurfrules` | `.windsurfrules` presence | Directive-imperative | ✅ Full support |
| **Kiro** | `adapters/kiro/manifest.json` → `.kiro/manifest.json` | `.kiro/` presence | Structured-checklist | ✅ Full support |

## Feature Matrix

| Feature | Claude Code | Gemini CLI | Cursor | Copilot | Codex | OpenCode | Windsurf | Kiro |
|---|---|---|---|---|---|---|---|---|
| Skill auto-loading | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Model variants | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Slash commands | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ |
| Agent personas | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Subagent dispatch | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ | ❌ | ✅ |
| Browser DevTools MCP | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Git worktrees | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Permission enforcement | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ⚠️ |

**Legend:** ✅ = Supported | ⚠️ = Partial (prompt-based enforcement) | ❌ = Not supported

## Notes

### Permission Enforcement
All platforms currently use **prompt-based enforcement** — permissions are declared in skill frontmatter and enforced through behavioral instructions, not platform-level access controls. True platform-level enforcement is a roadmap item pending platform API support.

### Slash Commands
Some platforms (Copilot, Codex, OpenCode) don't have native slash command support. On these platforms, users can invoke commands by saying "run the /spec command" or by referencing the command file directly.

### Subagent Dispatch
Subagent support requires the platform to support spawning independent agent sessions. Currently supported on: Claude Code (native subagent), Cursor (background agents), Gemini CLI (parallel sessions), Codex (multi-turn), and Kiro (agent hooks).

### Browser Testing
Browser DevTools MCP integration requires: Claude Code (native), Gemini CLI (MCP plugin), or Cursor (MCP plugin). Other platforms cannot inspect browser state directly.
