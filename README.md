# 🤲 CodeHands

> *Named after Super Smash Bros' "Master Hand" and economics' "The Invisible Hand" — people pursuing their own self-interests (vibe coding or otherwise) promote general social good. The good is more structured, higher-quality code through the invisible guidance of checks and systems.*

**Production-grade engineering skills for AI coding agents.** CodeHands provides 31 composable skills that cover the entire software development lifecycle — from brainstorming to shipping — with built-in security, multi-agent orchestration, and deterministic model detection.

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

---

## Why CodeHands?

AI coding agents are powerful, but without structure they produce inconsistent, insecure, untested code. CodeHands is the invisible hand that guides agents toward engineering excellence:

- **🧪 Strong TDD** — No production code without a failing test first. Delete-first policy for code-before-tests.
- **🔒 Security-First** — OWASP Top 10, LLM Security Top 10, Agentic Security Top 10 — all covered.
- **🤖 Multi-Agent Orchestration** — Subagent dispatch with two-stage review (spec compliance → code quality).
- **🎯 Model-Aware** — Deterministic model detection adapts skill behavior to Claude, Gemini, GPT, and more.
- **⚡ Task Decomposition Engine** — Every request is classified, assessed, and routed before any code is written.
- **📊 3-Tier Context System** — Load only what's needed. 86% context savings on quick fixes.

---

## Performance

### Context Efficiency

CodeHands' tier system only loads what's needed per session, dramatically reducing context window consumption:

| Scenario | Skills Loaded | Context Cost | Savings vs. All Skills |
|---|---|---|---|
| **Quick Fix** (typo, bug) | 3 | ~5,500 tokens | **86% saved** |
| **Session Start** (Tier 0 only) | 5 | ~8,200 tokens | **79% saved** |
| **Refactor** | 7 | ~12,000 tokens | **70% saved** |
| **New Feature** (full lifecycle) | 13 | ~19,700 tokens | **50% saved** |
| **All Skills** (worst case) | 31 | ~39,900 tokens | baseline |

> A quick bug fix loads only ~5,500 tokens of guidance — less than a single long prompt — while still enforcing TDD, verification, and classification.

### Validation Speed

| Validator | Time | Skills Checked |
|---|---|---|
| Bash scripts (3 separate) | 1,357 ms | 31 + cross-refs |
| **`codehands validate` (Rust CLI)** | **61 ms** | 31 + cross-refs |
| **Speedup** | **22x faster** | — |

### Guardrail Coverage

Every skill ships with behavioral enforcement — not just instructions, but resistance to common shortcuts:

| Guardrail | Count | What It Does |
|---|---|---|
| Anti-rationalization entries | **319** | Factual rebuttals to every excuse an agent uses to skip steps |
| Verification checklist items | **222** | Concrete pass/fail criteria — nothing ships without evidence |
| Red flag indicators | **889** | Warning signals that trigger intervention |
| Risk escalation signals | **8 categories** | Auth, payments, data, infra changes auto-escalate review |

### Framework Comparison

| Capability | No Framework | CodeHands |
|---|---|---|
| Request classification | ❌ Agent guesses | ✅ 7-category engine with fast paths |
| Task decomposition | ❌ Ad hoc | ✅ Mandatory decomposition gate |
| TDD enforcement | ❌ Optional | ✅ Iron Law — no code without failing test |
| Security review | ❌ Often skipped | ✅ Auto-escalation for high-risk changes |
| Session continuity | ❌ Starts from scratch | ✅ State recovery + progress journaling |
| Context efficiency | ❌ Load everything | ✅ 3-tier system, 50-86% reduction |
| Codebase awareness | ❌ Agent reads ad hoc | ✅ Systematic reconnaissance with cached profiles |
| Workflow presets | ❌ Manual selection | ✅ 9 built-in presets, custom presets supported |
| Platform support | ❌ Single platform | ✅ 7 adapters (Claude, Gemini, Cursor, Copilot, Codex, Windsurf, Kiro) |
| Model-aware enforcement | ❌ One-size-fits-all | ✅ Collaborative/checklist/directive per model |
| CLI tooling | ❌ None | ✅ Rust CLI — init, validate, detect, skills, presets, audit, status |

### By the Numbers

| Metric | Value |
|---|---|
| Active skills | **31** |
| Workflow presets | **9** |
| Platform adapters | **7** |
| Slash commands | **8** |
| Agent personas | **5** |
| Security references | **2** (OWASP LLM + Agentic Top 10) |
| Behavioral guardrails | **1,430** (319 anti-rationalizations + 222 checklists + 889 red flags) |
| Zero dependencies | Pure markdown — no server, no runtime, no database |

---

## Quick Start

#### _Do the manual install, I haven't gotten this into the marketplaces for Claude, Cursor, Gemini, etc._

### Claude Code
```bash
claude install codehands
```

### Cursor
Install from the Cursor plugin marketplace → search "CodeHands"

### Gemini CLI
```bash
cp adapters/GEMINI.md ./GEMINI.md
```

### Rust CLI (Any Platform)
```bash
# From the CodeHands repo
cd cli && cargo build --release
./target/release/codehands init --platform claude
```

### Manual Install
```bash
git clone https://github.com/codehands/codehands.git
cp -r codehands/skills/ ./.codehands/skills/
cp codehands/adapters/CLAUDE.md ./CLAUDE.md  # or your platform adapter
```

## Architecture

```
★ TASK-DECOMPOSITION-ENGINE (always first)
  ↓ classifies → assesses → reconnoiters → decomposes → routes
  ↓
DEFINE → PLAN → BUILD → ORCHESTRATE → VERIFY → REVIEW → SHIP → GOVERN
 /spec   /plan  /build                /test    /review  /ship   /audit
```

### 31 Skills Across 9 Phases

| Phase | Skills |
|---|---|
| **Engine** | task-decomposition-engine, codebase-reconnaissance, risk-assessment-and-escalation |
| **Define** | brainstorming, spec-driven-development |
| **Plan** | planning-and-task-breakdown, context-engineering |
| **Build** | test-driven-development, incremental-implementation, source-driven-development, frontend-ui-engineering, api-and-interface-design, using-git-worktrees |
| **Orchestrate** | multi-agent-orchestration, executing-plans |
| **Verify** | systematic-debugging, browser-testing-with-devtools, verification-before-completion |
| **Review** | code-review-and-quality, code-simplification, security-and-hardening, performance-optimization |
| **Ship** | finishing-a-development-branch, git-workflow-and-versioning, ci-cd-and-shipping, deprecation-and-migration, documentation-and-adrs |
| **Govern** | agent-security, audit-and-governance |
| **Meta** | writing-skills, using-codehands |

### Skill Tier System

| Tier | When Loaded | Count | Context Cost |
|---|---|---|---|
| **Tier 0 — Always On** | Every session | 5 | ~8,200 tokens |
| **Tier 1 — Phase-Activated** | When the engine routes to a phase | 15 | ~12,000 tokens (typical) |
| **Tier 2 — Specialist** | Only when detected or requested | 11 | loaded on demand |

### 9 Workflow Presets

| Preset | Skills | Overhead |
|---|---|---|
| `quick-fix` | 3 | ~30 seconds |
| `new-feature` | 14 | Full lifecycle |
| `refactor` | 7 | Lightweight planning |
| `security-audit` | 5 | ~10 min thorough scan |
| `performance-review` | 4 | Baseline measurement first |
| `api-development` | 9 | API-specific guidance |
| `frontend-build` | 9 | Includes a11y + responsive |
| `ship-release` | 5 | Pre-launch + deployment |
| `debug-session` | 4 | 4-phase investigation |

### 5 Agent Personas

- **code-reviewer** — Two-stage review: spec compliance then code quality
- **test-engineer** — Test pyramid, DAMP, Beyonce Rule
- **security-auditor** — OWASP-aligned with severity matrix and PoC requirements
- **performance-engineer** — Core Web Vitals, profiling, bundle analysis
- **architect** — Design review, SOLID, component boundaries

### 7 Platform Adapters

Claude Code · Cursor · Gemini CLI · GitHub Copilot · Codex · OpenCode · Windsurf · Kiro

## How It Works

Every skill is a `SKILL.md` file with:
- **YAML frontmatter** — name, version, permissions, triggers, tier, model variants
- **Standard sections** — Overview, When to Use, Process, Common Rationalizations, Red Flags, Verification
- **Anti-rationalization tables** — Prevent agents from cutting corners with factual rebuttals
- **Model variants** — Adapt enforcement language to Claude, Gemini, GPT automatically

Skills are loaded by your agent platform's native instruction mechanism. Zero runtime dependencies. Maximum portability.

## CLI

```bash
codehands init --platform claude   # Set up CodeHands in your project
codehands validate                 # Validate all skills (22x faster than bash)
codehands detect                   # Show detected platform and enforcement style
codehands skills --tier 0          # List always-on skills
codehands presets                  # Show available workflow presets
codehands audit --format json      # Generate audit report
codehands status                   # Show project CodeHands status
```

## Slash Commands

| Command | Phase | Skills Activated |
|---|---|---|
| `/spec` | Define | brainstorming + spec-driven-development |
| `/plan` | Plan | planning-and-task-breakdown + context-engineering |
| `/build` | Build | TDD + incremental-implementation + source-driven-development |
| `/test` | Verify | verification-before-completion + browser-testing |
| `/review` | Review | code-review + security + performance |
| `/ship` | Ship | git-workflow + ci-cd-and-shipping |
| `/debug` | Debug | systematic-debugging |
| `/audit` | Govern | audit-and-governance |

## Inspiration & Credits

CodeHands is built from the ground up, inspired by the best ideas from:

- Multi-agent orchestration,TDD requirements, systematic debugging, git worktrees, full SDLC coverage, security engineering, performance optimization, context engineering, and goofy engineering cultures that needed a gentle nudge in the right direction

We are deeply grateful to pioneers like Jesse Vincent, Addy Osmani, and many others (Danyelle, Kris, Saro, Jerry, Blanco, Toverton) for creating this space.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines. CodeHands is community-governed under Apache 2.0 — all contributions welcome.

## License

Apache 2.0 — See [LICENSE](LICENSE)

