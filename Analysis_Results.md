# CodeHands: Deep Analysis & Engineering PRD

> *Named after Super Smash Bros' "Master Hand" and economics' "The Invisible Hand" — the principle that people pursuing their own self-interests (vibe coding or otherwise) promote general social good. In this case, the good is more structured, higher-quality code through the invisible guidance of checks and systems.*

---

## Part 1: Deep Individual Analysis

---

### 1.1 — obra/superpowers

> **"An agentic skills framework & software development methodology that works."**

| Dimension | Detail |
|---|---|
| **Author** | Jesse Vincent (obra) / Prime Radiant |
| **Stars** | 148k ⭐ |
| **Forks** | 12.7k |
| **Commits** | 421 |
| **Contributors** | 28 |
| **License** | MIT |
| **Version** | v5.0.7 (Mar 31, 2026) |
| **Languages** | Shell (58.8%), JavaScript (29.6%), HTML (4.3%), Python (3.7%), TypeScript (2.8%) |
| **Platform Support** | Claude Code, Cursor, Codex, OpenCode, Gemini CLI, GitHub Copilot CLI |

#### Architecture

Superpowers is a **workflow-centric** framework built around a linear development lifecycle:

```
Brainstorm → Git Worktree → Plan → Execute (Subagent) → TDD → Code Review → Branch Finish
```

Each skill is a `SKILL.md` file with YAML frontmatter (`name`, `description`) and structured markdown. Skills are **automatically triggered** based on the agent's current activity — they are "mandatory workflows, not suggestions."

#### Directory Structure
```
superpowers/
├── .claude-plugin/        # Claude Code plugin config
├── .codex/                # Codex harness adapter
├── .cursor-plugin/        # Cursor plugin config
├── .opencode/             # OpenCode harness adapter
├── agents/                # Agent configuration
├── commands/              # Slash commands
├── docs/                  # Documentation
├── hooks/                 # Lifecycle hooks
├── scripts/               # Utility scripts (JS, Shell, Python)
├── skills/                # Core skill definitions
│   ├── brainstorming/
│   ├── using-git-worktrees/
│   ├── writing-plans/
│   ├── subagent-driven-development/
│   ├── executing-plans/
│   ├── dispatching-parallel-agents/
│   ├── test-driven-development/
│   ├── systematic-debugging/
│   ├── verification-before-completion/
│   ├── requesting-code-review/
│   ├── receiving-code-review/
│   ├── finishing-a-development-branch/
│   ├── writing-skills/
│   └── using-superpowers/
├── tests/                 # Skill tests
├── AGENTS.md              # Codex agent instructions
├── CLAUDE.md              # Claude Code instructions
└── GEMINI.md              # Gemini CLI instructions
```

#### Core Skills (14 total)

| Category | Skills |
|---|---|
| **Collaboration** | brainstorming, writing-plans, executing-plans, dispatching-parallel-agents, requesting-code-review, receiving-code-review, using-git-worktrees, finishing-a-development-branch, subagent-driven-development |
| **Testing** | test-driven-development |
| **Debugging** | systematic-debugging, verification-before-completion |
| **Meta** | writing-skills, using-superpowers |

#### Key Strengths

1. **Deep Subagent Orchestration** — The `subagent-driven-development` skill is the crown jewel. It defines a complete multi-agent architecture: fresh subagent per task, implementer → spec reviewer → code quality reviewer pipeline, model selection by task complexity, and structured status handling (DONE, DONE_WITH_CONCERNS, NEEDS_CONTEXT, BLOCKED).

2. **Ironclad TDD Enforcement** — "NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST." The skill demands deletion of code written before tests, with no "keep as reference" exceptions. This is the most aggressive TDD enforcement of any agent framework.

3. **Git Worktree Integration** — Unique feature: automatic creation of isolated workspaces on new branches, with setup verification and clean test baselines before implementation starts.

4. **Brainstorming as Hard Gate** — The `<HARD-GATE>` tag prevents any implementation until a design has been presented and approved. Includes visual companion for browser-based mockups.

5. **Systematic Debugging Methodology** — Four-phase root cause investigation with scientific method hypothesis testing. Includes explicit "3+ fix failure → question architecture" escalation.

6. **Human-Partner Framing** — Uses "your human partner" language throughout, framing the AI as a collaborative partner rather than a tool. This shapes agent behavior toward collaboration.

7. **Mature Plugin Ecosystem** — Available via official Claude marketplace, Cursor marketplace, and direct install on 6+ platforms.

8. **Heavy PR Quality Control** — 94% PR rejection rate with explicit anti-slop guidelines. Skills are treated as "code that shapes agent behavior."

#### Key Weaknesses

1. **No Security Skill** — Zero dedicated security coverage. No OWASP references, no input validation guidance, no secrets management skill, no security review process.

2. **No Performance Skill** — No coverage for performance optimization, benchmarking, or load testing.

3. **No CI/CD Skill** — No guidance on continuous integration, deployment pipelines, or release automation.

4. **No Frontend-Specific Skills** — No UI engineering, accessibility, or browser testing guidance.

5. **No API Design Skill** — No guidance on interface design, versioning, or contract testing.

6. **No Documentation Skill** — No structured approach to ADRs, changelogs, or technical writing.

7. **Narrow Skill Coverage** — 14 skills heavily weighted toward workflow orchestration. Thin on actual engineering practices beyond TDD and debugging.

8. **Shell-Heavy Implementation** — 58.8% Shell makes cross-platform support fragile. Windows support requires Batchfile wrappers.

9. **No Reference Checklists** — Unlike agent-skills, no standalone security/performance/accessibility/testing reference documents.

10. **No Agent Personas** — No pre-configured specialist agents for targeted reviews.

---

### 1.2 — addyosmani/agent-skills

> **"Production-grade engineering skills for AI coding agents."**

| Dimension | Detail |
|---|---|
| **Author** | Addy Osmani (Google VP of Engineering) |
| **Stars** | 13.3k ⭐ |
| **Forks** | 1.6k |
| **Commits** | 116 |
| **Contributors** | <10 (loading error on page) |
| **License** | MIT |
| **Version** | 0.5.0 (Apr 10, 2026) |
| **Languages** | Shell (100%) |
| **Platform Support** | Claude Code, Cursor, Gemini CLI, Windsurf, OpenCode, GitHub Copilot, Kiro |

#### Architecture

Agent-skills is a **lifecycle-centric** framework built around a full SDLC pipeline:

```
DEFINE → PLAN → BUILD → VERIFY → REVIEW → SHIP
  /spec   /plan  /build  /test   /review  /ship
```

Seven slash commands map to development phases. Skills activate both through commands and contextually based on activity (e.g., building UI triggers `frontend-ui-engineering`).

#### Directory Structure
```
agent-skills/
├── .claude-plugin/           # Claude Code plugin config
├── .claude/commands/         # 7 slash commands
├── .github/workflows/        # CI/CD workflows
├── agents/                   # 3 specialist personas
│   ├── code-reviewer.md
│   ├── test-engineer.md
│   └── security-auditor.md
├── docs/                     # Setup guides per tool
├── hooks/                    # Session lifecycle hooks
├── references/               # 4 supplementary checklists
│   ├── testing-patterns.md
│   ├── security-checklist.md
│   ├── performance-checklist.md
│   └── accessibility-checklist.md
├── skills/                   # 20 core skills
│   ├── idea-refine/
│   ├── spec-driven-development/
│   ├── planning-and-task-breakdown/
│   ├── incremental-implementation/
│   ├── test-driven-development/
│   ├── context-engineering/
│   ├── source-driven-development/
│   ├── frontend-ui-engineering/
│   ├── api-and-interface-design/
│   ├── browser-testing-with-devtools/
│   ├── debugging-and-error-recovery/
│   ├── code-review-and-quality/
│   ├── code-simplification/
│   ├── security-and-hardening/
│   ├── performance-optimization/
│   ├── git-workflow-and-versioning/
│   ├── ci-cd-and-automation/
│   ├── deprecation-and-migration/
│   ├── documentation-and-adrs/
│   ├── shipping-and-launch/
│   └── using-agent-skills/
├── AGENTS.md
├── CLAUDE.md
└── CONTRIBUTING.md
```

#### Core Skills (20 total)

| Phase | Skills |
|---|---|
| **Define** | idea-refine, spec-driven-development |
| **Plan** | planning-and-task-breakdown |
| **Build** | incremental-implementation, test-driven-development, context-engineering, source-driven-development, frontend-ui-engineering, api-and-interface-design |
| **Verify** | browser-testing-with-devtools, debugging-and-error-recovery |
| **Review** | code-review-and-quality, code-simplification, security-and-hardening, performance-optimization |
| **Ship** | git-workflow-and-versioning, ci-cd-and-automation, deprecation-and-migration, documentation-and-adrs, shipping-and-launch |

#### Key Strengths

1. **Comprehensive SDLC Coverage** — 20 skills span the entire development lifecycle from idea refinement to production deployment, deprecation, and migration.

2. **Security-First Engineering** — Dedicated `security-and-hardening` skill covering OWASP Top 10, injection prevention, authentication/authorization, XSS, CSP, CORS, rate limiting, secrets management, file upload safety, and npm audit triage. Backed by `references/security-checklist.md`.

3. **Performance Engineering** — Dedicated `performance-optimization` skill with Core Web Vitals (LCP, CLS, INP), bundle analysis, runtime profiling, and a performance checklist.

4. **Agent Personas** — Three pre-configured specialist agents: `code-reviewer`, `test-engineer`, `security-auditor`. Each has structured output formats and scoped review guidelines.

5. **Reference Checklists** — Four standalone reference documents (testing, security, performance, accessibility) that skills can pull in on demand, keeping token usage minimal through progressive disclosure.

6. **Context Engineering** — Unique skill dedicated to optimizing agent context: 5-level hierarchy (Rules → Specs → Source → Errors → History), token management, confusion management patterns, MCP integrations, and anti-patterns.

7. **Google Engineering Culture Baked In** — Explicitly incorporates concepts from _Software Engineering at Google_: Hyrum's Law (API design), Beyonce Rule (testing), Chesterton's Fence (simplification), trunk-based development, Shift Left, feature flags.

8. **Anti-Rationalization Tables** — Every skill includes a structured table of common excuses agents use to skip steps, paired with factual rebuttals. This is a powerful behavioral shaping mechanism.

9. **Formal Skill Anatomy** — `docs/skill-anatomy.md` provides a complete specification for skill structure: frontmatter, standard sections, writing principles, naming conventions, cross-referencing rules.

10. **CI/CD & Shipping** — Skills for CI/CD automation, deprecation/migration planning, documentation/ADRs, and launch readiness checks.

11. **Browser Testing** — DevTools integration for DOM inspection, console monitoring, network analysis, and screenshot-based visual regression testing. Includes security boundaries for browser-read data.

12. **Broader Platform Support** — Setup guides for 7+ platforms including Kiro and Windsurf.

#### Key Weaknesses

1. **No Subagent Orchestration** — No equivalent to Superpowers' subagent-driven-development. Brief mention of subagents in TDD but no formal multi-agent coordination skill.

2. **No Git Worktree Skill** — No automated workspace isolation or branch management.

3. **Weaker Brainstorming** — `idea-refine` is simpler than Superpowers' brainstorming with its hard gates, visual companion, and spec self-review loops.

4. **No Parallel Agent Dispatch** — No skill for coordinating concurrent subagent workflows.

5. **Less Battle-Tested** — 116 commits vs 421, much newer project (v0.5.0 vs v5.0.7), smaller community.

6. **No Test Suite for Skills** — No `tests/` directory for validating skill behavior.

7. **No Verification-Before-Completion** — No dedicated skill ensuring the agent actually confirms its work before declaring success.

8. **Shell-Only Implementation** — 100% Shell limits extensibility and testing.

9. **Weaker TDD Enforcement** — TDD skill is comprehensive but less aggressive than Superpowers' "delete code written before tests" iron law.

10. **No Visual Companion** — No browser-based mockup/diagram tool during design phase.

---

## Part 2: Exhaustive Comparison Table

| # | Dimension | obra/superpowers | addyosmani/agent-skills | Winner |
|---|---|---|---|---|
| **Meta** |||||
| 1 | **Stars** | 148k | 13.3k | Superpowers |
| 2 | **Maturity** | v5.0.7, 421 commits, 28 contributors | v0.5.0, 116 commits, <10 contributors | Superpowers |
| 3 | **Philosophy** | Workflow methodology + skills | Engineering practices as skills | Complementary |
| 4 | **Core Focus** | Agent orchestration & process discipline | Full SDLC engineering practices | Complementary |
| 5 | **License** | MIT | MIT | Tie |
| **Architecture** |||||
| 6 | **Skill Format** | SKILL.md + YAML frontmatter | SKILL.md + YAML frontmatter | Tie (compatible) |
| 7 | **Skill Count** | 14 | 20 (+3 personas, +4 references) | Agent-skills |
| 8 | **Lifecycle Coverage** | Brainstorm → Plan → Execute → Test → Review → Branch | Define → Plan → Build → Verify → Review → Ship | Agent-skills (broader) |
| 9 | **Slash Commands** | Implicit (via skill auto-invocation) | 7 explicit (/spec, /plan, /build, /test, /review, /code-simplify, /ship) | Agent-skills |
| 10 | **Plugin/Extension System** | Claude marketplace, Cursor marketplace, gemini-extension.json | Claude marketplace, per-tool setup guides | Superpowers |
| 11 | **Cross-Referencing** | `superpowers:<skill-name>` namespace | Plain skill name references | Superpowers (namespaced) |
| **Orchestration** |||||
| 12 | **Subagent Architecture** | Full subagent-driven-development with implementer → spec reviewer → quality reviewer pipeline | Mentioned briefly (TDD subagent for test isolation) | **Superpowers** (massive gap) |
| 13 | **Model Selection** | Explicit guidance (cheap/standard/capable by task complexity) | None | Superpowers |
| 14 | **Parallel Execution** | dispatching-parallel-agents skill | None | Superpowers |
| 15 | **Task Status Handling** | DONE, DONE_WITH_CONCERNS, NEEDS_CONTEXT, BLOCKED | None | Superpowers |
| 16 | **Context Isolation** | Fresh subagent per task (explicit) | N/A | Superpowers |
| **Development Process** |||||
| 17 | **Brainstorming/Design** | Deep: hard gate, visual companion, spec self-review, 2-3 alternatives, Socratic dialogue | Basic: idea-refine + spec-driven-development (separate skills) | Superpowers |
| 18 | **Planning** | Bite-sized tasks (2-5 min), exact file paths, complete code, verification steps | Task breakdown with dependencies, estimations | Tie |
| 19 | **TDD Enforcement** | Iron Law: "Delete code written before tests. No exceptions. Delete means delete." | Strong but less aggressive: "Write a failing test before writing the code" | Superpowers (more extreme) |
| 20 | **Debugging** | 4-phase systematic, root-cause tracing, defense-in-depth, 3+ fix → architecture question | 2-skill approach: debugging-and-error-recovery | Superpowers |
| 21 | **Code Review** | Two skills: requesting + receiving. Two-stage: spec compliance then quality | Single comprehensive skill: code-review-and-quality | Superpowers (more structured) |
| **Engineering Practices** |||||
| 22 | **Security** | ❌ None | ✅ Full: OWASP Top 10, injection, XSS, CSP, auth, rate limiting, secrets, npm audit triage | **Agent-skills** (massive gap) |
| 23 | **Performance** | ❌ None | ✅ Core Web Vitals, profiling, bundle analysis | **Agent-skills** |
| 24 | **Accessibility** | ❌ None | ✅ Reference checklist | **Agent-skills** |
| 25 | **Frontend/UI** | ❌ None | ✅ frontend-ui-engineering skill | **Agent-skills** |
| 26 | **API Design** | ❌ None | ✅ api-and-interface-design with Hyrum's Law | **Agent-skills** |
| 27 | **CI/CD** | ❌ None | ✅ ci-cd-and-automation skill | **Agent-skills** |
| 28 | **Documentation/ADRs** | ❌ None | ✅ documentation-and-adrs skill | **Agent-skills** |
| 29 | **Deprecation/Migration** | ❌ None | ✅ deprecation-and-migration skill | **Agent-skills** |
| 30 | **Code Simplification** | Implied in refactoring | ✅ Dedicated skill with Chesterton's Fence | Agent-skills |
| 31 | **Context Engineering** | Implicit (subagent context crafting) | ✅ Dedicated skill: 5-level hierarchy, MCP integrations, confusion management | **Agent-skills** |
| **Quality Assurance** |||||
| 32 | **Test Pyramid** | Via TDD skill (basic) | Explicit pyramid: unit (80%) → integration (15%) → E2E (5%) + test sizes (small/medium/large) + Beyonce Rule | Agent-skills |
| 33 | **Browser Testing** | ❌ None | ✅ DevTools MCP: DOM, console, network, performance, screenshots, security boundaries | **Agent-skills** |
| 34 | **Anti-Rationalization** | Strong in TDD and debugging skills | Systematic in every skill (mandatory section) | Agent-skills (more consistent) |
| 35 | **Red Flags Section** | Present in most skills | Present in every skill (mandatory section) | Tie |
| 36 | **Verification Checklists** | verification-before-completion skill | Verification section in every skill | Tie (different approaches) |
| **Git & SCM** |||||
| 37 | **Git Worktrees** | ✅ Dedicated skill: isolated workspace, branch setup, test baseline | ❌ None | **Superpowers** |
| 38 | **Branch Management** | ✅ finishing-a-development-branch: merge/PR/keep/discard | Basic in git-workflow-and-versioning | Superpowers |
| 39 | **Git Workflow** | Worktree-focused | ✅ Trunk-based development, feature flags, versioning | Agent-skills (broader) |
| **Extensibility** |||||
| 40 | **Skill Authoring Guide** | writing-skills skill | docs/skill-anatomy.md + CONTRIBUTING.md | Tie (different formats) |
| 41 | **Skill Testing** | ✅ tests/ directory | ❌ None | Superpowers |
| 42 | **Agent Personas** | ❌ None | ✅ 3 personas: code-reviewer, test-engineer, security-auditor | **Agent-skills** |
| 43 | **Reference Documents** | Testing anti-patterns (inline) | ✅ 4 standalone: testing, security, performance, accessibility | **Agent-skills** |
| 44 | **Progressive Disclosure** | Supporting files per skill directory | References at project root + SKILL.md entry points | Tie |
| **Platform & Community** |||||
| 45 | **Platform Count** | 6 (Claude, Cursor, Codex, OpenCode, Gemini, Copilot) | 7+ (Claude, Cursor, Gemini, Windsurf, OpenCode, Copilot, Kiro) | Agent-skills |
| 46 | **PR Process** | 94% rejection rate, aggressive anti-slop | Standard CONTRIBUTING.md | Superpowers (higher bar) |
| 47 | **Community** | Discord, GitHub Issues, release announcements | GitHub Issues, PRs | Superpowers |
| 48 | **Documentation** | README + per-platform READMEs | README + per-platform docs/ guides | Tie |

### Synthesis: Complementary Strengths

```
                    ┌─────────────────────────────────────────┐
                    │          SUPERPOWERS EXCELS AT           │
                    │                                         │
                    │  • Multi-agent orchestration            │
                    │  • Subagent task dispatch & review       │
                    │  • Git worktree isolation                │
                    │  • Brainstorming with hard gates         │
                    │  • Aggressive TDD enforcement            │
                    │  • Systematic debugging methodology      │
                    │  • Model selection optimization          │
                    │  • Community maturity (148k stars)        │
                    └─────────────┬───────────────────────────┘
                                  │
                    ┌─────────────▼───────────────────────────┐
                    │         SHARED / OVERLAPPING             │
                    │                                         │
                    │  • SKILL.md format (YAML frontmatter)   │
                    │  • TDD workflows                         │
                    │  • Code review processes                 │
                    │  • Anti-rationalization tables            │
                    │  • Planning and task breakdown            │
                    │  • Red flags and verification sections   │
                    │  • MIT license                            │
                    │  • Multi-platform support                 │
                    └─────────────┬───────────────────────────┘
                                  │
                    ┌─────────────▼───────────────────────────┐
                    │        AGENT-SKILLS EXCELS AT            │
                    │                                         │
                    │  • Security engineering (OWASP)          │
                    │  • Performance optimization              │
                    │  • Frontend/UI engineering                │
                    │  • API design (Hyrum's Law)              │
                    │  • CI/CD and automation                   │
                    │  • Documentation and ADRs                 │
                    │  • Deprecation and migration              │
                    │  • Context engineering                    │
                    │  • Agent personas                         │
                    │  • Reference checklists                   │
                    │  • Browser testing with DevTools          │
                    │  • Shipping and launch readiness          │
                    │  • Accessibility                          │
                    │  • Google engineering culture              │
                    └─────────────────────────────────────────┘
```

---

## Part 3: Engineering-Grade PRD — "CodeHands"

### A Unified Agent Skills Framework for Developers

---

### 3.0 Executive Summary

**CodeHands** is a proposed open-source (Apache 2.0), community-governed agent skills framework that merges the multi-agent orchestration depth of Superpowers with the full-SDLC engineering coverage of Agent-skills, while addressing the security, observability, and extensibility gaps in both. Written from the ground up with modern enhancements — not a fork of either project.

The framework provides:
- **30+ composable skills** covering the entire software development lifecycle
- **Multi-agent orchestration** with task dispatch, review pipelines, and model optimization
- **Security-first engineering** aligned with OWASP LLM Top 10 and Agentic Security Top 10
- **A plugin ecosystem** with OIDC-based Trusted Publishing, dependency pinning, and permission manifests
- **Observable, auditable agent behavior** with structured logging, audit dashboards, and human-in-the-loop gates
- **Deterministic model detection** with platform-aware hooks for LLM-specific skill tuning

---

### 3.1 Problem Statement

AI coding agents are becoming the primary interface through which developers build software. However, current agent skills frameworks have critical gaps:

1. **Fragmented coverage**: Superpowers excels at orchestration but lacks security, performance, and shipping skills. Agent-skills covers the full SDLC but has no multi-agent coordination.
2. **No unified security posture**: Neither framework addresses OWASP LLM Top 10, agentic security risks (prompt injection, tool misuse, memory poisoning), or skill supply chain integrity.
3. **No observability**: Neither framework provides structured logging, audit trails, or telemetry for agent actions.
4. **No formal plugin governance**: Skills are distributed as raw markdown files with no integrity verification, permission scoping, or version pinning.
5. **No cross-framework interoperability**: Each framework has its own namespace, invocation patterns, and platform adapters.

---

### 3.2 Goals & Non-Goals

#### Goals
- G1: Provide **30+ production-grade skills** covering Define → Plan → Build → Verify → Review → Ship
- G2: Provide **a multi-agent orchestration layer** with subagent dispatch, review pipelines, model selection, and context isolation
- G3: Provide **security-first engineering** at three layers: application code, agent behavior, and skill supply chain
- G4: Provide **a plugin/skill ecosystem** with OIDC Trusted Publishing, cryptographic signing, permission manifests, and dependency pinning
- G5: Provide **structured observability** with audit logging, audit dashboards/reports, and human-in-the-loop approval workflows
- G6: Support **8+ agent platforms** (Claude Code, Cursor, Gemini CLI, Copilot, Codex, OpenCode, Windsurf, Kiro)
- G7: Provide **deterministic model detection hooks** to adapt skill output in real-time to the active LLM
- G8: Release under **Apache 2.0 license** with **community governance**

#### Non-Goals
- NG1: Building a new AI agent runtime or LLM inference engine
- NG2: Replacing MCP servers or tool registries
- NG3: Providing language-specific frameworks (React, Django, etc.) — skills must be language-agnostic
- NG4: Commercializing the tool — CodeHands is free and open source for everyone

---

### 3.3 Users & Personas

| Persona | Description | Key Needs |
|---|---|---|
| **Solo Developer** | Individual using AI agent for personal/freelance projects | Quick setup, sensible defaults, low friction |
| **Team Lead** | Engineering lead managing a team using agents | Consistent quality, policy enforcement, audit trails |
| **Security Engineer** | AppSec professional responsible for agent-produced code | Vulnerability scanning, secrets detection, OWASP compliance |
| **Platform Engineer** | Maintains agent infrastructure for an org | Plugin governance, permission management, telemetry |
| **Skill Author** | Community contributor creating new skills | Clear spec, testing tools, publishing pipeline |

---

### 3.4 Functional Requirements

#### FR1: Skill Engine

| ID | Requirement | Priority | Source |
|---|---|---|---|
| FR1.1 | Skills are SKILL.md files with YAML frontmatter (`name`, `description`, `version`, `permissions`, `triggers`) | P0 | Both |
| FR1.2 | Skills auto-activate based on trigger conditions (contextual) AND manual invocation (slash commands) | P0 | Both |
| FR1.3 | Skills follow a standard anatomy: Overview, When to Use, Process, Rationalizations, Red Flags, Verification | P0 | Agent-skills |
| FR1.4 | Skills support progressive disclosure: SKILL.md entry point + supporting files loaded on demand | P0 | Both |
| FR1.5 | Skills can cross-reference other skills via namespaced identifiers (`codehands:<skill-name>`) | P0 | Superpowers |
| FR1.6 | Skills have a `version` field following semver | P1 | New |
| FR1.7 | Skills declare required `permissions` (filesystem, network, shell, secrets) | P0 | New (OWASP AST10) |
| FR1.8 | Skills declare `platform_compatibility` to indicate which agent runtimes they support | P1 | New |

#### FR2: Core Skill Library (30+ skills)

##### Define Phase
| ID | Skill | Source |
|---|---|---|
| FR2.1 | `brainstorming` — Socratic design refinement with hard gates, visual companion, spec self-review | Superpowers |
| FR2.2 | `spec-driven-development` — Formal specification authoring with exit criteria | Agent-skills |

##### Plan Phase
| ID | Skill | Source |
|---|---|---|
| FR2.3 | `planning-and-task-breakdown` — Task decomposition with 2-5 minute granularity, file paths, verification steps | Both (merged) |
| FR2.4 | `context-engineering` — 5-level context hierarchy, MCP integrations, confusion management | Agent-skills |

##### Build Phase
| ID | Skill | Source |
|---|---|---|
| FR2.5 | `test-driven-development` — Iron Law enforcement: failing test → minimal code → refactor. Delete-first policy for code-before-tests | Superpowers (stricter) + Agent-skills (test pyramid, DAMP, Beyonce Rule) |
| FR2.6 | `incremental-implementation` — Small, verifiable steps with continuous integration | Agent-skills |
| FR2.7 | `source-driven-development` — Read existing code before writing, follow established patterns | Agent-skills |
| FR2.8 | `frontend-ui-engineering` — Component architecture, accessibility, responsive design | Agent-skills |
| FR2.9 | `api-and-interface-design` — Hyrum's Law, versioning, contract testing, backward compatibility | Agent-skills |
| FR2.10 | `using-git-worktrees` — Isolated workspace on new branch, setup verification, clean baseline | Superpowers |

##### Orchestrate Phase (NEW — from Superpowers)
| ID | Skill | Source |
|---|---|---|
| FR2.11 | `subagent-driven-development` — Full multi-agent pipeline: implementer → spec reviewer → quality reviewer | Superpowers |
| FR2.12 | `dispatching-parallel-agents` — Concurrent subagent workflows for independent tasks | Superpowers |
| FR2.13 | `executing-plans` — Batch execution with human checkpoints for parallel sessions | Superpowers |

##### Verify Phase
| ID | Skill | Source |
|---|---|---|
| FR2.14 | `systematic-debugging` — 4-phase root cause investigation, hypothesis testing, 3+ fix escalation | Superpowers |
| FR2.15 | `browser-testing-with-devtools` — DevTools MCP: DOM, console, network, performance, screenshots | Agent-skills |
| FR2.16 | `verification-before-completion` — Mandatory proof that work is actually done | Superpowers |

##### Review Phase
| ID | Skill | Source |
|---|---|---|
| FR2.17 | `code-review-and-quality` — Two-stage: spec compliance then code quality | Superpowers structure + Agent-skills content |
| FR2.18 | `code-simplification` — Chesterton's Fence, complexity reduction | Agent-skills |
| FR2.19 | `security-and-hardening` — OWASP Top 10, injection, XSS, CSP, auth, secrets, npm audit | Agent-skills |
| FR2.20 | `performance-optimization` — Core Web Vitals, profiling, bundle analysis | Agent-skills |

##### Ship Phase
| ID | Skill | Source |
|---|---|---|
| FR2.21 | `finishing-a-development-branch` — Merge/PR/keep/discard decision with test verification | Superpowers |
| FR2.22 | `git-workflow-and-versioning` — Trunk-based development, feature flags, semantic versioning | Agent-skills |
| FR2.23 | `ci-cd-and-automation` — Pipeline configuration, Shift Left, deployment gates | Agent-skills |
| FR2.24 | `deprecation-and-migration` — Code-as-liability, migration planning, sunset timelines | Agent-skills |
| FR2.25 | `documentation-and-adrs` — Architecture Decision Records, changelog, API docs | Agent-skills |
| FR2.26 | `shipping-and-launch` — Launch checklists, rollback plans, monitoring setup | Agent-skills |

##### Governance Phase (NEW)
| ID | Skill | Source |
|---|---|---|
| FR2.27 | `agent-security-posture` — OWASP LLM Top 10, prompt injection defense, output guardrails | New (OWASP) |
| FR2.28 | `skill-supply-chain-integrity` — Skill verification, signed publishers, dependency pinning | New (OWASP AST10) |
| FR2.29 | `audit-and-compliance` — Structured audit logging, GDPR/SOC2 awareness, data handling policies | New |

##### Meta
| ID | Skill | Source |
|---|---|---|
| FR2.30 | `writing-skills` — Skill authoring guide with testing methodology | Superpowers |
| FR2.31 | `using-codehands` — Framework introduction and onboarding | Both |
| FR2.32 | `audit-and-reporting` — Audit skill that generates dashboards and structured reports on all skill activity outputs | New |

#### FR3: Multi-Agent Orchestration

| ID | Requirement | Priority |
|---|---|---|
| FR3.1 | Support subagent dispatch with isolated context per task | P0 |
| FR3.2 | Implement two-stage review pipeline: spec compliance → code quality | P0 |
| FR3.3 | Support four implementer statuses: DONE, DONE_WITH_CONCERNS, NEEDS_CONTEXT, BLOCKED | P0 |
| FR3.4 | Provide model selection guidance: cheap/standard/capable by task complexity | P1 |
| FR3.5 | Support parallel agent dispatch for independent tasks with conflict prevention | P1 |
| FR3.6 | Support prompt templates for implementer, spec reviewer, and code quality reviewer | P0 |
| FR3.7 | Enforce "fresh subagent per task" to prevent context pollution | P0 |

#### FR4: Agent Personas

| ID | Requirement | Priority |
|---|---|---|
| FR4.1 | `code-reviewer` persona with structured output format and severity classification | P0 |
| FR4.2 | `test-engineer` persona for testing strategy and coverage analysis | P0 |
| FR4.3 | `security-auditor` persona with OWASP-aligned review scope and severity matrix | P0 |
| FR4.4 | `performance-engineer` persona for profiling and optimization reviews | P1 |
| FR4.5 | `architect` persona for design review and pattern analysis | P1 |
| FR4.6 | Custom persona authoring via `agents/<name>.md` | P1 |

#### FR5: Reference Library

| ID | Requirement | Priority |
|---|---|---|
| FR5.1 | `testing-patterns.md` — Framework-specific patterns, anti-patterns, test doubles | P0 |
| FR5.2 | `security-checklist.md` — Pre-commit, auth, authz, input, headers, CORS, OWASP | P0 |
| FR5.3 | `performance-checklist.md` — Loading, runtime, network, build optimization | P0 |
| FR5.4 | `accessibility-checklist.md` — WCAG 2.1 AA compliance, ARIA, keyboard nav | P0 |
| FR5.5 | `testing-anti-patterns.md` — Mock abuse, snapshot abuse, flaky tests | P1 |
| FR5.6 | `owasp-llm-top10.md` — LLM-specific vulnerabilities and mitigations | P0 |
| FR5.7 | `owasp-agentic-top10.md` — Agent-specific security risks | P0 |

---

### 3.5 Security Requirements

> [!CAUTION]
> Security is the primary gap in both existing frameworks and the primary differentiator for AgentForge.

#### SR1: Application Security (Code the Agent Produces)

| ID | Requirement | Priority |
|---|---|---|
| SR1.1 | All skills that generate code MUST include relevant security guidance inline | P0 |
| SR1.2 | The `security-and-hardening` skill MUST cover OWASP Top 10 2025 with code examples | P0 |
| SR1.3 | Input validation with schema libraries (Zod, Joi) MUST be demonstrated in build skills | P0 |
| SR1.4 | Parameterized queries MUST be enforced — string concatenation for SQL is a red flag | P0 |
| SR1.5 | Secrets MUST never appear in generated code, logs, or skill output | P0 |
| SR1.6 | Security headers (CSP, HSTS, X-Frame-Options) MUST be included in web deployment skills | P0 |
| SR1.7 | Authentication/authorization patterns MUST follow current best practices (bcrypt ≥12, httpOnly, secure, sameSite) | P0 |
| SR1.8 | npm/pip audit MUST be integrated into the CI/CD skill | P0 |

#### SR2: Agentic Security (The Agent's Own Behavior)

| ID | Requirement | Priority |
|---|---|---|
| SR2.1 | Agents MUST treat all browser-read data as untrusted (DOM, console, network) | P0 |
| SR2.2 | Agents MUST NOT interpret external content as instructions (prompt injection defense) | P0 |
| SR2.3 | Agents MUST NOT access cookies, localStorage tokens, or credentials via JS execution without explicit human approval | P0 |
| SR2.4 | Agents MUST surface ambiguity to the human instead of silently guessing | P0 |
| SR2.5 | Agents MUST NOT execute destructive operations (file deletion, database drops, production deploys) without human-in-the-loop approval | P0 |
| SR2.6 | Agent actions MUST be logged in a structured format for audit | P1 |
| SR2.7 | Context files loaded from external sources MUST be treated as untrusted data, not instructions | P0 |

#### SR3: Skill Supply Chain Security

| ID | Requirement | Priority |
|---|---|---|
| SR3.1 | Published skills MUST be signed by their author using GPG or equivalent | P1 |
| SR3.2 | Skill installations MUST verify signatures before activation | P1 |
| SR3.3 | Skills MUST declare a `permissions` manifest listing required capabilities (fs.read, fs.write, net.http, shell.exec) | P0 |
| SR3.4 | Users MUST be prompted to approve permissions on first install | P0 |
| SR3.5 | Skill dependencies MUST be pinned to specific versions (no floating ranges) | P1 |
| SR3.6 | A `skill audit` command MUST scan installed skills for known vulnerabilities | P2 |
| SR3.7 | Skills from unverified publishers MUST display a warning on install | P0 |

---

### 3.6 Non-Functional Requirements

#### NFR1: Performance

| ID | Requirement | Target |
|---|---|---|
| NFR1.1 | Skill loading time | < 100ms per skill |
| NFR1.2 | Context token overhead for skill activation | < 2,000 tokens per skill (progressive disclosure) |
| NFR1.3 | Reference documents loaded only when referenced | Zero overhead when unused |
| NFR1.4 | Total framework context footprint | < 5,000 tokens for default config |

#### NFR2: Compatibility

| ID | Requirement | Target |
|---|---|---|
| NFR2.1 | Platform support | 8+ agent runtimes |
| NFR2.2 | OS support | macOS, Linux, Windows (WSL) |
| NFR2.3 | Backward compatibility with existing Superpowers skills | Full (migration guide provided) |
| NFR2.4 | Backward compatibility with existing Agent-skills skills | Full (migration guide provided) |
| NFR2.5 | SKILL.md format stability | Guaranteed for 2 major versions |

#### NFR3: Extensibility

| ID | Requirement | Target |
|---|---|---|
| NFR3.1 | Custom skill creation via documented anatomy spec | < 30 min to create a new skill |
| NFR3.2 | Custom persona creation via `agents/<name>.md` | < 10 min to create a persona |
| NFR3.3 | Plugin marketplace with search, install, update | CLI-based |
| NFR3.4 | Org-private skill registries for enterprise use | Supported via config |

#### NFR4: Observability

| ID | Requirement | Target |
|---|---|---|
| NFR4.1 | Structured JSON logs for all skill activations | On by default |
| NFR4.2 | Audit trail for human-in-the-loop decisions | Persisted to `.codehands/audit/` |
| NFR4.3 | Telemetry opt-in for skill usage analytics | Disabled by default |
| NFR4.4 | Error reporting with context snapshots | On skill failure |
| NFR4.5 | `audit-and-reporting` skill generates dashboards and reports on all activity outputs | On demand via `/audit` command |

---

### 3.7 Skill Specification (AgentForge SKILL.md v2)

```yaml
---
# Required
name: skill-name-with-hyphens
description: >
  What this skill does (third person). Use when [trigger conditions].
  Maximum 1024 characters.
version: 1.0.0

# Recommended
permissions:
  - fs.read          # Read project files
  - fs.write         # Write/modify project files
  - shell.exec       # Execute shell commands
  - net.http         # Make HTTP requests
  - browser.inspect  # Read browser state via DevTools

triggers:
  - context: "implementing any feature"
  - context: "fixing a bug"
  - command: "/build"
  - command: "/test"

platforms:
  - claude-code
  - cursor
  - gemini-cli
  - copilot
  - codex
  - opencode
  - windsurf
  - kiro

dependencies:
  - test-driven-development@^1.0.0
  - context-engineering@^1.0.0

# Model-specific tuning (loaded dynamically via model detection hooks)
model_variants:
  claude:
    enforcement_style: "human-partner collaborative"
  gemini:
    enforcement_style: "structured-checklist"
  gpt:
    enforcement_style: "directive-imperative"

author: codehands-core
signed: true
---

## Overview
[What + Why — 1-2 sentences]

## When to Use
[Positive triggers + Negative exclusions]

## Process
[Step-by-step workflow with code examples, decision trees, and verification gates]

## Common Rationalizations
| Rationalization | Reality |
|---|---|
| [Excuse] | [Rebuttal] |

## Red Flags
- [Behavioral pattern indicating violation]

## Verification
- [ ] [Exit criterion with evidence requirement]

## See Also
- [Cross-references to related skills and references]
```

---

### 3.8 Architectural Decisions

#### AD1: Files-as-Interface, Not Runtime

AgentForge is a collection of markdown files, not a runtime. Skills are loaded by the agent platform's native skill/instruction mechanism. This ensures:
- Zero runtime dependencies
- Maximum platform compatibility
- Minimal attack surface
- Simple contribution model (edit markdown)

#### AD2: Namespaced Skill References

Skills reference each other via `codehands:<skill-name>` to prevent collisions with org-specific or third-party skills. Migration from `superpowers:` and `agent-skills:` namespaces is mandatory — CodeHands is written from the ground up, not a fork.

#### AD3: Permission-First Skill Loading

Skills declare permissions upfront. Agent platforms SHOULD enforce these permissions; if the platform doesn't support permission enforcement, CodeHands adds a human-readable warning to the skill output. There is no benefit to building a standalone runtime for permission enforcement — the declarative approach keeps the framework zero-dependency and maximally portable. Enforcement is the platform's responsibility.

#### AD7: Deterministic Model Detection

CodeHands includes a model detection hook that determines which LLM the user is querying. This is NOT done by asking the model "what are you?" (unreliable). Instead:
1. **API metadata inspection** — Read the model identifier from the agent platform's configuration (e.g., `model_name` from the LLM client instance)
2. **Platform-specific hooks** — Each platform adapter exposes a `get_active_model()` function that reads from the platform's config files (CLAUDE.md → Claude, GEMINI.md → Gemini, etc.)
3. **Skill variant selection** — Based on the detected model, load the appropriate `model_variants` section from the skill's frontmatter to adjust enforcement language, examples, and behavioral shaping in real-time

#### AD8: Audit Skill for Dashboards & Reports

The `audit-and-reporting` skill generates structured reports and dashboards covering:
- All skill activations across sessions (which skills fired, when, outcomes)
- Human-in-the-loop decision audit trail
- Code quality metrics over time (test coverage, security findings, review outcomes)
- Model usage and cost tracking (if telemetry opted in)
- Export formats: Markdown report, JSON data, HTML dashboard

#### AD4: Progressive Disclosure by Default

Main SKILL.md files are optimized for token efficiency. Reference materials, checklists, and examples are stored in separate files and loaded only when referenced, keeping default context overhead < 2,000 tokens per skill.

#### AD5: Anti-Rationalization as a First-Class Feature

Every skill MUST include a Common Rationalizations table. This is not optional documentation — it is behavioral shaping code. The table is the primary mechanism for preventing agents from cutting corners.

#### AD6: Human-In-The-Loop as Default for Destructive Actions

All skills that involve destructive operations (file deletion, database modification, production deployment, permission changes) MUST include human approval gates. Skills MAY allow opt-out for trusted/automated environments.

---

### 3.9 Delivery Roadmap

#### Phase 1: Foundation (Weeks 1-4)

| Deliverable | Description |
|---|---|
| Skill Engine Spec v2 | Finalized SKILL.md format with permissions, versioning, triggers |
| Core Skills (20) | Merged best-of-both from Superpowers + Agent-skills |
| Platform Adapters (4) | Claude Code, Cursor, Gemini CLI, Copilot |
| Reference Library (5) | Security, testing, performance, accessibility, OWASP LLM |
| Agent Personas (3) | Code reviewer, test engineer, security auditor |
| Migration Guide | From Superpowers and Agent-skills |

#### Phase 2: Orchestration (Weeks 5-8)

| Deliverable | Description |
|---|---|
| Subagent Skills (3) | subagent-driven-development, parallel-agents, executing-plans |
| Model Selection Guide | Cost/capability optimization matrix |
| Prompt Templates (3) | Implementer, spec reviewer, quality reviewer |
| Skill Testing Framework | Adversarial eval harness for skill behavior validation |
| Platform Adapters (+4) | Codex, OpenCode, Windsurf, Kiro |

#### Phase 3: Security & Governance (Weeks 9-12)

| Deliverable | Description |
|---|---|
| Agent Security Skill | OWASP LLM Top 10 + Agentic Security Top 10 |
| Skill Supply Chain | Signing, verification, permission manifests |
| Audit Logging | Structured JSON logs for skill activations and decisions |
| Human-in-the-Loop | Approval workflow for destructive operations |
| Governance Skills (3) | agent-security-posture, skill-supply-chain, audit-compliance |

#### Phase 4: Ecosystem (Weeks 13-16)

| Deliverable | Description |
|---|---|
| Plugin Marketplace | CLI-based search, install, update, audit |
| Org-Private Registries | Enterprise skill hosting |
| Telemetry (Opt-in) | Usage analytics for skill improvement |
| Community Templates | Starter kits for common project types |
| v1.0 Release | Stable API, format guarantee |

---

### 3.10 Success Metrics

| Metric | Target | Measurement |
|---|---|---|
| Test coverage of agent-produced code | > 80% (when TDD skill active) | Pre/post comparison |
| Security vulnerabilities in agent-produced code | 50% reduction vs. unguided agents | SAST scan comparison |
| Time to first commit (new project) | < 30 min from brainstorming to first passing test | Workflow timing |
| Skill adoption rate | > 50% of eligible activations within 30 days | Telemetry (opt-in) |
| Platform support | 8+ agent runtimes with feature parity | Compatibility matrix |
| Community growth | 1,000+ skills in marketplace within 6 months | Registry metrics |
| Supply chain incidents | Zero skill-borne vulnerabilities | Incident tracking |

---

### 3.11 Resolved Decisions

| # | Decision | Resolution |
|---|---|---|
| 1 | **Governance** | Community-governed (Apache Foundation model) |
| 2 | **License** | Apache 2.0 |
| 3 | **Naming** | "CodeHands" — entirely new identity |
| 4 | **Namespace** | Force migration to `codehands:` — written from the ground up, not a fork |
| 5 | **Runtime enforcement** | No — remain purely declarative (files-only). No benefit to a standalone runtime. |
| 6 | **Commercial model** | No — fully open source. Audit dashboards included as a skill, not a paid tier. |
| 7 | **LLM-specific tuning** | Yes — `model_variants` in frontmatter + deterministic model detection hooks |
| 8 | **Certification** | Yes — OIDC-based Trusted Publishing (see §3.11.1) |

#### 3.11.1 Skill Certification: Trusted Publishing Programs

CodeHands will adopt the industry-standard **OIDC-based Trusted Publishing** model used by major registries:

| Registry | Program | Key Security Feature |
|---|---|---|
| **npm** | Trusted Publishing (GA) | Automatic provenance attestations via OpenSSF |
| **PyPI** | Trusted Publishers | OIDC links identity to source repository (45k+ projects) |
| **crates.io** | Trusted Publishing | Ephemeral, workflow-scoped tokens (30min TTL) |
| **Docker Hub** | Verified Publishers | Organization verification + signed images |
| **GitHub Actions** | Artifact Attestations | Sigstore-based provenance for any artifact |
| **Homebrew** | Community audit | No formal program — relies on open-source peer review |

**CodeHands implementation:**
1. Skill authors configure their GitHub/GitLab CI to publish via OIDC — no long-lived API tokens
2. Published skills receive automatic **provenance attestations** proving where and how they were built
3. Skills with valid attestations display a ✅ verified badge in the marketplace
4. Unverified skills display a ⚠️ warning on install
5. Enterprise orgs can require verified-only skills via policy config

---

### 3.12 Risk Register

*Exported to separate file: [CodeHands_Risk_Register.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/CodeHands_Risk_Register.md)*

---

### Appendix A: OWASP Alignment Matrix

| OWASP Framework | Relevant CodeHands Coverage |
|---|---|
| **OWASP Top 10 (Web)** | `security-and-hardening` skill + `security-checklist.md` |
| **OWASP Top 10 for LLM (2025)** | `agent-security-posture` skill + `owasp-llm-top10.md` |
| **OWASP Agentic Security Top 10** | `agent-security-posture` + `skill-supply-chain-integrity` |
| **OWASP Agentic Skills Top 10** | `skill-supply-chain-integrity` + permission manifests |

### Appendix B: Platform Compatibility Matrix

| Feature | Claude Code | Cursor | Gemini CLI | Copilot | Codex | OpenCode | Windsurf | Kiro |
|---|---|---|---|---|---|---|---|---|
| SKILL.md loading | ✅ | ✅ | ✅ | ✅ (via `.github/copilot-instructions.md`) | ✅ (via `AGENTS.md`) | ✅ (via `AGENTS.md`) | ✅ (via `.windsurfrules`) | ✅ (via native skills) |
| Slash commands | ✅ (native) | ✅ (via rules) | ✅ (native) | ✅ (via custom commands) | ❌ (prompt-only) | ✅ (via `skill` tool) | ✅ (via rules) | ✅ (native skills) |
| Subagent dispatch | ✅ (native) | ✅ (Agent mode) | ✅ (native) | ✅ (multi-agent via Copilot Extensions) | ✅ (native sandbox) | ⚠️ (limited) | ⚠️ (limited, via Cascade) | ✅ (native subagents) |
| Plugin marketplace | ✅ (official) | ✅ (plugin system) | ✅ (extensions) | ✅ (Copilot Extensions store) | ❌ (prompt-only) | ❌ (manual config) | ❌ (manual config) | ⚠️ (emerging) |
| Permission enforcement | ⚠️ (prompt-based) | ⚠️ (prompt-based) | ⚠️ (prompt-based) | ⚠️ (prompt-based) | ✅ (sandbox isolation) | ⚠️ (prompt-based) | ⚠️ (prompt-based) | ⚠️ (prompt-based) |
| DevTools MCP | ✅ (via MCP) | ✅ (via MCP) | ✅ (via MCP) | ⚠️ (limited MCP support) | ❌ | ⚠️ (limited MCP) | ⚠️ (limited MCP) | ⚠️ (emerging MCP) |
| Model detection | ✅ (API metadata) | ✅ (config file) | ✅ (API metadata) | ✅ (config) | ✅ (API metadata) | ✅ (config) | ✅ (config) | ✅ (config) |

*(✅ = Full support, ⚠️ = Partial/limited, ❌ = Not supported)*

> **Note:** All platforms support SKILL.md loading via their respective instruction file formats. Model detection is achievable on all platforms by reading the platform's configuration rather than asking the model. Permission enforcement is currently prompt-based on most platforms (declarative, not runtime-enforced), with Codex being the exception via its sandboxed execution environment. MCP support is broadly available on Claude Code, Cursor, and Gemini CLI via the Linux Foundation's Agentic AI Foundation standards.
