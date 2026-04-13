# CodeHands v1.1.0 — Base Workflow Engine

## What Changed

The framework previously had 33 skills covering the full SDLC, but **nothing forced the lifecycle to execute**. Skills were advisory — an agent could jump straight to coding and only encounter planning if a trigger context happened to match.

The **Task Decomposition Engine** fixes this by serving as the `main()` function of CodeHands — a root orchestrator that activates on **every request** and routes through the correct skill chain before any code is written.

---

## Three New Skills Added

### 1. Task Decomposition Engine (★ Root Orchestrator)
[SKILL.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/skills/task-decomposition-engine/SKILL.md) | [templates.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/skills/task-decomposition-engine/templates.md)

The spine of CodeHands. Every request flows through 6 phases:

```
Phase 0: Session State Recovery  → Resume or fresh start?
Phase 1: Request Classification  → Question / Trivial / Bug / Feature / Refactor / Investigation / Config
Phase 2: Complexity Assessment   → Scope × Risk × Uncertainty
Phase 3: Codebase Reconnaissance → Language, patterns, test framework, build system
Phase 4: Decomposition Gate      → Does this need a plan? Fast path or full lifecycle?
Phase 5: Skill Chain Assembly    → Build the specific sequence of skills for this task
Phase 6: Progress Journaling     → Record decisions, progress, and next-session state
```

**Intelligent behaviors built into the engine:**

| Behavior | What It Does | Developer Benefit |
|---|---|---|
| **Request Classification** | Sorts every request into 7 categories with distinct lifecycle paths | Trivial fixes skip planning; features get full rigor. No wasted overhead. |
| **Complexity Assessment** | Evaluates scope × risk × uncertainty on a 3-axis matrix | High-risk changes (auth, payments) get elevated review automatically. |
| **Fast Paths** | Trivial fixes and bug fixes bypass spec/plan phases | ~30 seconds of classification overhead, not 10 minutes of unnecessary ceremony. |
| **Session Recovery** | Checks for in-progress plans and session state on startup | Multi-session projects resume exactly where they left off. |
| **Classify-Up Rule** | When in doubt, classify higher (Feature over Trivial) | Prevents scope explosions from underestimated work. |
| **"Just" Detector** | "Just add auth" is still classified as Feature, not Trivial | Natural language shortcuts don't bypass engineering rigor. |
| **Progress Journaling** | Records decisions, rationale, and state to `.codehands/session-state.md` | Survives context resets, provides audit trail, readable by the user. |
| **Skill Chain Presentation** | Tells the user the planned approach before starting | Transparency — user can adjust the approach. |

### 2. Codebase Reconnaissance
[SKILL.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/skills/codebase-reconnaissance/SKILL.md)

Systematically profiles a project before modifications:
- **7-point scan:** Language, architecture, tests, build, patterns, CI/CD, dependencies
- **Cached profile:** Saved to session state, not repeated per task
- **Dependency audit:** Runs `npm audit` / `pip audit` / `cargo audit`
- **Pattern detection:** Identifies naming, error handling, async style, import conventions

### 3. Risk Assessment & Escalation
[SKILL.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/skills/risk-assessment-and-escalation/SKILL.md)

Automatic risk detection and review escalation:
- **Signal scanning:** Detects auth, payment, data, and infrastructure changes by file pattern
- **4-tier risk levels:** Low → Medium → High → Critical
- **Auto-escalation:** High risk triggers security auditor; Critical triggers architect + mandatory human gate
- **Risk report:** Attached to plan/PR with signals, escalations, and mitigation requirements

---

## Adapters Updated

All 7 platform adapters now include the ROOT ORCHESTRATOR section pointing to the engine as the mandatory first step:

| Adapter | Added |
|---|---|
| [CLAUDE.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/adapters/CLAUDE.md) | Full orchestrator block + updated skill directory with Engine category |
| [GEMINI.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/adapters/GEMINI.md) | Orchestrator with checklist-style intake |
| [AGENTS.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/adapters/AGENTS.md) | Orchestrator with MUST/SHALL directive |
| [copilot-instructions.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/adapters/copilot-instructions.md) | Root Orchestrator section |
| [.windsurfrules](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/adapters/.windsurfrules) | Root Orchestrator section |

---

## Updated Inventory

| Category | Count |
|---|---|
| **Total files** | **89** |
| Skills (SKILL.md) | 36 (35 active + 1 template) |
| Supporting files | 1 (templates.md in engine) |
| Agent personas | 5 |
| Reference docs | 7 |
| Slash commands | 8 |
| Platform adapters | 7 |
| Documentation | 13 |
| Test scripts | 3 |

## Test Results

```
✅ Skill Format Validation:    35/35 passed, 0 warnings
✅ Frontmatter Schema:          35/35 passed
✅ Cross-Reference Resolution:  33/33 resolved
```

---

## Architecture: Before vs. After

````carousel
### Before: Advisory Skills
```
User Request → Agent decides which skills to follow (maybe)
                     ↓
              Writes code directly (often skipping planning)
```
Skills existed but nothing enforced their activation.
<!-- slide -->
### After: Mandatory Engine
```
User Request → ★ TASK-DECOMPOSITION-ENGINE (always)
                     ↓
              Classify → Assess → Reconnoiter → Decompose → Route
                     ↓
              Correct skill chain assembled and executed
                     ↓
              Progress journaled for session continuity
```
The engine IS the entry point. Skills are no longer optional.
````
