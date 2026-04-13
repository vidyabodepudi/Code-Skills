# CodeHands — Engineering Review

## Question 1: Are Skills Optimized for Single Responsibility?

### TL;DR: Mostly yes. Five pairs have overlap worth merging.

I audited all 35 active skills for content overlap, responsibility bleed, and duplication. Here's what I found:

### ✅ Clean Single-Responsibility Skills (25)

These skills do one thing well with no overlap:

| Skill | Responsibility | Verdict |
|---|---|---|
| task-decomposition-engine | Root request classifier and router | ✅ Unique |
| codebase-reconnaissance | Project profiling before changes | ✅ Unique |
| risk-assessment-and-escalation | Risk detection and review escalation | ✅ Unique |
| brainstorming | Socratic design exploration | ✅ Unique |
| spec-driven-development | RFC 2119 formal requirements | ✅ Unique |
| planning-and-task-breakdown | Task decomposition | ✅ Unique |
| context-engineering | Context window management | ✅ Unique |
| test-driven-development | TDD Iron Law | ✅ Unique |
| incremental-implementation | One-behavior-per-increment | ✅ Unique |
| source-driven-development | Read-before-write | ✅ Unique |
| frontend-ui-engineering | Component/responsive/a11y | ✅ Unique |
| api-and-interface-design | REST/API conventions | ✅ Unique |
| using-git-worktrees | Isolated workspaces | ✅ Unique |
| executing-plans | Plan execution coordination | ✅ Unique |
| systematic-debugging | 4-phase root cause investigation | ✅ Unique |
| verification-before-completion | Evidence-based completion gate | ✅ Unique |
| code-review-and-quality | Two-stage review pipeline | ✅ Unique |
| performance-optimization | Measure-first optimization | ✅ Unique |
| code-simplification | Targeted refactoring | ✅ Unique |
| documentation-and-adrs | ADR format + docs standards | ✅ Unique |
| deprecation-and-migration | Safe deprecation flow | ✅ Unique |
| writing-skills | Skill authoring guide | ✅ Unique |
| using-codehands | Framework onboarding | ✅ Unique |
| browser-testing-with-devtools | DevTools MCP verification | ✅ Unique |
| git-workflow-and-versioning | Conventional commits + semver | ✅ Unique |

### ⚠️ Five Merge Candidates (10 skills → 5)

| Current Skills | Overlap | Recommendation |
|---|---|---|
| `audit-and-compliance` + `audit-and-reporting` | Both triggered by `/audit`, both deal with compliance data. Compliance defines WHAT to check, reporting defines HOW to present it. But in practice, you'd never run one without the other. | **Merge → `audit-and-governance`** (~1,200 tokens combined, under budget) |
| `subagent-driven-development` + `dispatching-parallel-agents` | Subagent covers strategy + review gates. Dispatch covers mechanics (scoping, tracking, conflict). They're always used together and dispatch is a dependent. | **Merge → `multi-agent-orchestration`** (~1,500 tokens combined, under budget) |
| `ci-cd-and-automation` + `shipping-and-launch` | CI/CD covers pipeline design. Shipping covers pre-launch checklists. But the pre-launch checklist IS the final CI/CD gate. They share the same trigger (`/ship`). | **Merge → `ci-cd-and-shipping`** (~1,200 tokens combined, under budget) |
| `agent-security-posture` + `skill-supply-chain-integrity` | Agent posture covers agent behavior boundaries. Supply chain covers skill verification. Both are about agent-level security (not code security). Both triggered by `/audit`. | **Merge → `agent-security`** (~1,500 tokens combined, under budget) |
| `finishing-a-development-branch` + `git-workflow-and-versioning` | Finishing covers merge/PR/discard decisions. Git workflow covers commits/semver. Both deal with git, both triggered by `/ship`. | **Keep separate** — these are genuinely distinct. Finishing is a one-time decision flow; git-workflow is an ongoing convention. |

### Impact of Merges

| Metric | Before | After |
|---|---|---|
| Active skills | 35 | **31** |
| Total context (all loaded) | ~43,560 tokens | **~39,760 tokens** (~8.7% reduction) |
| Skill chains (avg links) | 5-9 | 4-7 (fewer hops) |

> [!IMPORTANT]
> The 4th pair (agent-security-posture + skill-supply-chain-integrity) is debatable. They have distinct audiences: posture is for every session, supply chain is only for installing new skills. If you prefer to keep them separate for clarity, that's defensible.

---

## Question 2: Usability and Intelligent Automation

### TL;DR: Too many skills visible at once. Need a tiering system and presets.

### The Problem

With 35 skills, the agent faces a **selection problem** — even with the engine routing, the skill directory in the adapter file lists everything. This is:
- **Cognitively expensive** for the agent (large context to parse)
- **Confusing for users** who see the full catalog and don't know what's relevant
- **Token-wasteful** when skills are loaded that won't be needed

### Proposed Solution: Skill Tiers + Presets

#### Tier System

| Tier | When Loaded | Skills | ~Tokens |
|---|---|---|---|
| **Tier 0: Always On** | Every session, automatically | task-decomposition-engine, codebase-reconnaissance, test-driven-development, source-driven-development, verification-before-completion | ~8,000 |
| **Tier 1: Phase-Activated** | Loaded when the engine routes to a phase | brainstorming, spec, planning, incremental, code-review, security, debugging, git-workflow, finishing | ~12,000 |
| **Tier 2: Specialist** | Loaded only when explicitly needed or detected | frontend, API, browser-testing, performance, subagent, ci-cd, audit, deprecation, writing-skills | ~10,000 |

**Benefit:** A typical session only loads Tier 0 + 2-3 Tier 1 skills = ~12,000 tokens instead of ~43,000.

#### Preset System (Higher-Level Automation)

Instead of manually activating phases, presets bundle the most common workflows:

```yaml
# .codehands/presets.yml
presets:
  quick-fix:
    description: "Fix a bug or make a small change"
    auto_chain: [task-decomposition-engine, systematic-debugging, test-driven-development, verification-before-completion]
    skip_phases: [define, plan, review]

  new-feature:
    description: "Build a new feature from scratch"
    auto_chain: [task-decomposition-engine, codebase-reconnaissance, brainstorming, spec-driven-development, planning-and-task-breakdown, executing-plans, verification-before-completion, code-review-and-quality, finishing-a-development-branch]

  refactor:
    description: "Refactor existing code"
    auto_chain: [task-decomposition-engine, codebase-reconnaissance, source-driven-development, code-simplification, verification-before-completion]

  security-audit:
    description: "Full security review"
    auto_chain: [codebase-reconnaissance, security-and-hardening, agent-security-posture, audit-and-compliance]
```

The engine would then say:
> "This looks like a **quick-fix**. I'll follow the quick-fix preset: debug → test → verify. Estimated: 3 skills, no planning overhead."

#### Smart Defaults Based on `.codehands/config.yml`

```yaml
# Project-level defaults
defaults:
  # Skip brainstorming for this project (it's a library with clear patterns)
  skip_skills: [brainstorming]

  # Always include security review for this project (fintech)
  always_include: [security-and-hardening, risk-assessment-and-escalation]

  # Default preset when unclear
  default_preset: quick-fix
```

This means a fintech team **always gets security review** even on trivial fixes, while a rapid-prototyping team can **skip brainstorming** for well-defined library work.

---

## Question 3: Can We Make It Run Faster With a Language Change?

### TL;DR: CodeHands has no runtime to speed up. But the validation tooling can be rewritten.

### Why There's Nothing to Speed Up

CodeHands is **pure markdown files loaded by LLM platforms**. There is no:
- Server process
- Runtime engine
- HTTP proxy
- Database
- WebSocket connection

The "runtime" is the LLM itself reading SKILL.md files. The speed bottleneck is:
1. **LLM context loading** — how fast the model reads skill files (~milliseconds, not our control)
2. **Skill selection** — the engine's classification logic (happens in the model's reasoning, not our code)
3. **Test execution** — `npm test`, `cargo test`, etc. (depends on the user's project, not CodeHands)

### What CAN Be Optimized

The only executable code in CodeHands is the **test scripts** and **model detection**:

| Component | Current | Could Be | Speedup |
|---|---|---|---|
| `tests/skill-format.test.sh` | Bash (~2s) | Rust/Go binary | 10-50x faster |
| `tests/frontmatter.test.sh` | Bash (~2s) | Rust/Go binary | 10-50x faster |
| `tests/cross-reference.test.sh` | Bash (~1s) | Rust/Go binary | 10-50x faster |
| `scripts/model-detect.sh` | Bash (~0.1s) | Already instant | N/A |

But the total validation time is **~5 seconds**. Rewriting to Rust would make it sub-100ms — impressive but unnecessary.

### Where a Language Change WOULD Add Value

A **Rust/Go CLI binary** (`codehands-cli`) could provide:

```bash
# Validate all skills (instead of 3 separate bash scripts)
codehands validate

# Initialize CodeHands in a project (instead of manual cp)
codehands init --platform claude

# Detect model and display profile
codehands detect

# Generate audit report from project state
codehands audit --format json

# Skill search and selection
codehands skills --phase build

# Preset execution plan
codehands plan --preset new-feature
```

This would improve **developer experience** not by making CodeHands faster, but by:
1. Replacing manual `cp` setup with `codehands init`
2. Combining 3 test scripts into one `codehands validate` command
3. Providing `codehands audit` for programmatic report generation
4. Making skill discovery easier with `codehands skills`

> [!TIP]
> **Recommendation:** Build a lightweight Rust CLI (`codehands-cli`) as a v2.0 companion tool. It doesn't replace the markdown skills — it wraps the operational tooling around them. Think of it like `npm` wrapping `node` — the runtime is the LLM, the CLI is the package manager.

---

## Summary of Recommendations

| # | Recommendation | Priority | Impact |
|---|---|---|---|
| 1 | Merge 4 skill pairs (8 → 4) | **High** | -4 skills, -3,800 tokens, simpler tree |
| 2 | Add Skill Tier system (T0/T1/T2) | **High** | 70% context reduction per session |
| 3 | Add Preset system with `presets.yml` | **High** | One-word workflow activation |
| 4 | Add project-level defaults in `config.yml` | **Medium** | Team-specific customization |
| 5 | Build `codehands-cli` in Rust | **Low** | Better DX for init, validate, audit |
| 6 | Keep the markdown-first philosophy | — | No runtime to maintain, zero deps |
