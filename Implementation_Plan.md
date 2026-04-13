# CodeHands — Implementation Plan

## Goal

Build CodeHands from the ground up as a community-governed (Apache 2.0), unified agent skills framework that synthesizes the orchestration depth of `obra/superpowers` with the full-SDLC engineering coverage of `addyosmani/agent-skills`, while adding novel capabilities: deterministic model detection, OIDC-based Trusted Publishing, audit dashboards, and OWASP-aligned agentic security.

> [!IMPORTANT]
> CodeHands is NOT a fork or merge of either project. Each skill is rewritten from scratch with modern enhancements based on exhaustive analysis of both frameworks. The `codehands:` namespace is mandatory — no backward compatibility shims.

---

## User Review Required

> [!WARNING]
> **Skill Enhancement Strategy**: Per your directive to "do an exhaustive review of all the skills to hypothesize how we can improve them," Phase 1.2 includes a Skill Enhancement Audit for each of the 33 skills. This will add ~2 weeks to Phase 1 but ensures every skill is modernized, not just copied. Should this be a blocking prerequisite, or can we parallelize it with scaffold work?

> [!IMPORTANT]
> **Scope Confirmation**: This plan covers the full framework scaffold + all 33 skills + platform adapters + marketplace infrastructure. The estimated timeline is 16 weeks for a solo developer or 8 weeks with 2-3 contributors. Should we scope to an MVP (Phase 1 only: scaffold + P0 skills) first?

---

## Proposed Changes

### Phase 1: Foundation & Scaffold (Weeks 1-4)

---

#### Component 1: Project Scaffold

##### [NEW] Repository Structure
```
CodeHands/
├── LICENSE                          # Apache 2.0
├── README.md                        # Project overview, quick start
├── CONTRIBUTING.md                  # Contribution guidelines
├── CODE_OF_CONDUCT.md               # Community standards
├── CHANGELOG.md                     # Release history
│
├── .codehands/                      # Framework config directory
│   ├── config.yml                   # User config (model overrides, telemetry opt-in)
│   └── audit/                       # Audit log storage
│       └── .gitkeep
│
├── adapters/                        # Platform-specific adapter files
│   ├── CLAUDE.md                    # Claude Code instructions
│   ├── GEMINI.md                    # Gemini CLI instructions
│   ├── AGENTS.md                    # Codex / OpenCode instructions
│   ├── .cursor-plugin/              # Cursor plugin config
│   │   └── manifest.json
│   ├── .windsurfrules               # Windsurf rules
│   ├── copilot-instructions.md      # GitHub Copilot instructions
│   └── kiro/                        # Kiro native skills config
│       └── manifest.json
│
├── skills/                          # Core skill library (33 skills)
│   ├── _template/                   # Skill template for authors
│   │   └── SKILL.md
│   │
│   │ # ── DEFINE PHASE ──
│   ├── brainstorming/
│   │   ├── SKILL.md
│   │   └── visual-companion.md
│   ├── spec-driven-development/
│   │   └── SKILL.md
│   │
│   │ # ── PLAN PHASE ──
│   ├── planning-and-task-breakdown/
│   │   └── SKILL.md
│   ├── context-engineering/
│   │   └── SKILL.md
│   │
│   │ # ── BUILD PHASE ──
│   ├── test-driven-development/
│   │   ├── SKILL.md
│   │   └── test-pyramid-guide.md
│   ├── incremental-implementation/
│   │   └── SKILL.md
│   ├── source-driven-development/
│   │   └── SKILL.md
│   ├── frontend-ui-engineering/
│   │   └── SKILL.md
│   ├── api-and-interface-design/
│   │   └── SKILL.md
│   ├── using-git-worktrees/
│   │   └── SKILL.md
│   │
│   │ # ── ORCHESTRATE PHASE ──
│   ├── subagent-driven-development/
│   │   ├── SKILL.md
│   │   ├── implementer-prompt.md
│   │   ├── spec-reviewer-prompt.md
│   │   └── code-quality-reviewer-prompt.md
│   ├── dispatching-parallel-agents/
│   │   └── SKILL.md
│   ├── executing-plans/
│   │   └── SKILL.md
│   │
│   │ # ── VERIFY PHASE ──
│   ├── systematic-debugging/
│   │   └── SKILL.md
│   ├── browser-testing-with-devtools/
│   │   └── SKILL.md
│   ├── verification-before-completion/
│   │   └── SKILL.md
│   │
│   │ # ── REVIEW PHASE ──
│   ├── code-review-and-quality/
│   │   └── SKILL.md
│   ├── code-simplification/
│   │   └── SKILL.md
│   ├── security-and-hardening/
│   │   └── SKILL.md
│   ├── performance-optimization/
│   │   └── SKILL.md
│   │
│   │ # ── SHIP PHASE ──
│   ├── finishing-a-development-branch/
│   │   └── SKILL.md
│   ├── git-workflow-and-versioning/
│   │   └── SKILL.md
│   ├── ci-cd-and-automation/
│   │   └── SKILL.md
│   ├── deprecation-and-migration/
│   │   └── SKILL.md
│   ├── documentation-and-adrs/
│   │   └── SKILL.md
│   ├── shipping-and-launch/
│   │   └── SKILL.md
│   │
│   │ # ── GOVERNANCE PHASE ──
│   ├── agent-security-posture/
│   │   └── SKILL.md
│   ├── skill-supply-chain-integrity/
│   │   └── SKILL.md
│   ├── audit-and-compliance/
│   │   └── SKILL.md
│   ├── audit-and-reporting/
│   │   └── SKILL.md
│   │
│   │ # ── META ──
│   ├── writing-skills/
│   │   └── SKILL.md
│   └── using-codehands/
│       └── SKILL.md
│
├── agents/                          # Agent personas
│   ├── code-reviewer.md
│   ├── test-engineer.md
│   ├── security-auditor.md
│   ├── performance-engineer.md
│   └── architect.md
│
├── references/                      # Standalone checklists
│   ├── testing-patterns.md
│   ├── testing-anti-patterns.md
│   ├── security-checklist.md
│   ├── performance-checklist.md
│   ├── accessibility-checklist.md
│   ├── owasp-llm-top10.md
│   └── owasp-agentic-top10.md
│
├── commands/                        # Slash commands
│   ├── spec.md                      # /spec → Define phase
│   ├── plan.md                      # /plan → Plan phase
│   ├── build.md                     # /build → Build phase
│   ├── test.md                      # /test → Verify phase
│   ├── review.md                    # /review → Review phase
│   ├── ship.md                      # /ship → Ship phase
│   ├── audit.md                     # /audit → Audit & reporting
│   └── debug.md                     # /debug → Systematic debugging
│
├── hooks/                           # Session lifecycle hooks
│   ├── session-init.md              # Runs on session start
│   └── pre-commit.md               # Runs before git commit
│
├── scripts/                         # Utility scripts
│   ├── model-detect.sh              # Deterministic model detection
│   ├── skill-validate.sh            # SKILL.md frontmatter validator
│   ├── skill-audit.sh               # Check installed skills for issues
│   └── platform-check.sh            # Verify platform compatibility
│
├── tests/                           # Skill behavior tests
│   ├── README.md                    # Test methodology
│   ├── skill-format.test.sh         # Validate all SKILL.md conform to spec
│   ├── frontmatter.test.sh          # Validate YAML frontmatter
│   └── cross-reference.test.sh      # Validate all codehands: references resolve
│
├── docs/                            # Documentation
│   ├── skill-anatomy.md             # Canonical skill specification
│   ├── model-detection.md           # How model detection works
│   ├── trusted-publishing.md        # Marketplace certification guide
│   ├── platform-compatibility.md    # Verified compatibility matrix
│   ├── migration/
│   │   ├── from-superpowers.md      # Migration guide from Superpowers
│   │   └── from-agent-skills.md     # Migration guide from Agent-skills
│   └── setup/                       # Per-platform setup guides
│       ├── claude-code.md
│       ├── cursor.md
│       ├── gemini-cli.md
│       ├── copilot.md
│       ├── codex.md
│       ├── opencode.md
│       ├── windsurf.md
│       └── kiro.md
│
└── Analysis_Results.md              # Research & PRD (existing)
```

---

#### Component 2: Skill Specification v2

##### [NEW] [skill-anatomy.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/docs/skill-anatomy.md)
The canonical SKILL.md specification incorporating all resolved decisions:
- YAML frontmatter with `name`, `description`, `version`, `permissions`, `triggers`, `platforms`, `dependencies`, `model_variants`, `author`, `signed`
- Standard sections: Overview, When to Use, Process, Common Rationalizations, Red Flags, Verification, See Also
- Writing principles: Process over knowledge, specific over general, evidence over assumption, anti-rationalization as first-class, progressive disclosure, token-conscious
- Naming conventions: `lowercase-hyphen-separated` directories, `SKILL.md` uppercase, `codehands:<skill-name>` cross-references

##### [NEW] [_template/SKILL.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/skills/_template/SKILL.md)
Starter template for new skill authors with all required sections pre-filled.

---

#### Component 3: Skill Enhancement Audit

> [!IMPORTANT]
> Before writing each skill, perform an enhancement audit comparing the Superpowers and Agent-skills versions (where both exist) and documenting specific improvements. For skills unique to one framework, audit against industry best practices.

For each of the 33 skills, the audit process is:

1. **Source analysis** — Read both Superpowers and Agent-skills versions (if they exist)
2. **Gap identification** — What does each version miss?
3. **Enhancement hypothesis** — How can the CodeHands version improve on both?
4. **Model variant planning** — What should differ for Claude vs. Gemini vs. GPT?
5. **Write the enhanced skill** — From scratch, incorporating all improvements

**Priority order for skill writing:**

| Priority | Skills | Rationale |
|---|---|---|
| P0 - Week 1 | `using-codehands`, `test-driven-development`, `systematic-debugging`, `security-and-hardening` | Core identity + the 3 skills with the most impact on code quality |
| P0 - Week 2 | `brainstorming`, `spec-driven-development`, `planning-and-task-breakdown`, `context-engineering` | Define + Plan phases (must exist before Build) |
| P0 - Week 3 | `incremental-implementation`, `source-driven-development`, `code-review-and-quality`, `verification-before-completion` | Build + Verify core loop |
| P0 - Week 4 | `using-git-worktrees`, `finishing-a-development-branch`, `git-workflow-and-versioning`, `writing-skills` | Git workflow + meta skill for community |
| P1 - Week 5 | `subagent-driven-development`, `dispatching-parallel-agents`, `executing-plans` | Orchestration phase |
| P1 - Week 6 | `frontend-ui-engineering`, `api-and-interface-design`, `browser-testing-with-devtools` | Specialized build/verify |
| P1 - Week 7 | `performance-optimization`, `code-simplification`, `ci-cd-and-automation`, `shipping-and-launch` | Review + Ship |
| P2 - Week 8 | `documentation-and-adrs`, `deprecation-and-migration`, `agent-security-posture`, `skill-supply-chain-integrity`, `audit-and-compliance`, `audit-and-reporting` | Governance + remaining |

---

### Phase 2: Platform Adapters & Model Detection (Weeks 5-6)

---

#### Component 4: Platform Adapters

##### [NEW] adapters/CLAUDE.md
Claude Code instruction file that:
- Loads CodeHands skills via `CLAUDE.md` native format
- Maps 8 slash commands to skill invocations
- Configures session hooks

##### [NEW] adapters/GEMINI.md
Gemini CLI instruction file with equivalent functionality.

##### [NEW] adapters/AGENTS.md
Codex/OpenCode instruction file.

##### [NEW] adapters/.cursor-plugin/manifest.json
Cursor plugin manifest for marketplace distribution.

And similar for Copilot, Windsurf, and Kiro adapters.

**Per-adapter work:**
1. Translate CodeHands slash commands to platform-native invocation
2. Configure skill auto-activation triggers
3. Set up session lifecycle hooks
4. Verify SKILL.md loading behavior
5. Test model detection hook compatibility

---

#### Component 5: Model Detection

##### [NEW] [scripts/model-detect.sh](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/scripts/model-detect.sh)
Deterministic model detection script:

```bash
#!/bin/bash
# CodeHands Model Detection Hook
# Determines the active LLM by inspecting platform config — NOT by asking the model.

detect_model() {
    # Signal 1: Platform instruction file presence
    if [ -f "CLAUDE.md" ] || [ -f ".claude/settings.json" ]; then
        echo "claude"
    elif [ -f "GEMINI.md" ] || [ -f ".gemini/settings.json" ]; then
        echo "gemini"
    elif [ -f ".github/copilot-instructions.md" ]; then
        echo "copilot"
    elif [ -f "AGENTS.md" ]; then
        # Could be Codex or OpenCode — check further
        if [ -n "$CODEX_MODEL" ]; then
            echo "codex"
        else
            echo "opencode"
        fi
    elif [ -f ".windsurfrules" ]; then
        echo "windsurf"
    elif [ -d ".kiro" ]; then
        echo "kiro"
    fi

    # Signal 2: Environment variable override
    if [ -n "$CODEHANDS_MODEL" ]; then
        echo "$CODEHANDS_MODEL"
        return
    fi

    # Signal 3: Config file override
    if [ -f ".codehands/config.yml" ]; then
        model=$(grep "model:" .codehands/config.yml | awk '{print $2}')
        if [ -n "$model" ]; then
            echo "$model"
            return
        fi
    fi

    # Fallback: generic (no model-specific tuning applied)
    echo "generic"
}
```

##### [NEW] [docs/model-detection.md](file:///Users/vidyabodepudi/Documents/Code%20Projects/CodeHands/docs/model-detection.md)
Documentation covering:
- Why we don't ask the model "what are you?" (unreliable)
- The 3-signal detection hierarchy: env var → config file → platform file presence
- How `model_variants` in SKILL.md frontmatter are selected
- How to override via `.codehands/config.yml`
- Limitations and fallback behavior

---

### Phase 3: Agent Personas & References (Weeks 7-8)

---

#### Component 6: Agent Personas

##### [NEW] agents/code-reviewer.md
Enhanced code reviewer persona combining:
- Superpowers' two-stage review (spec compliance → quality)
- Agent-skills' structured output format with severity classification
- **Enhancement**: Model-specific review language variants

##### [NEW] agents/test-engineer.md
Test engineer persona with:
- Test pyramid analysis (80/15/5 split)
- DAMP over DRY guidance
- Beyonce Rule enforcement
- **Enhancement**: Framework-specific test patterns (Jest, pytest, Go testing)

##### [NEW] agents/security-auditor.md
Security auditor persona with:
- OWASP Top 10 review scope
- Severity classification (Critical/High/Medium/Low/Info)
- Proof of concept requirements for Critical/High
- **Enhancement**: OWASP LLM Top 10 and Agentic Security coverage (unique to CodeHands)

##### [NEW] agents/performance-engineer.md
New persona (not in either framework):
- Core Web Vitals analysis scope
- Bundle size budgets
- Runtime profiling guidance
- Database query optimization
- Memory leak detection

##### [NEW] agents/architect.md
New persona (not in either framework):
- Design review against SOLID, DRY, KISS
- Component boundary analysis
- Data flow and dependency review
- Scalability assessment

---

#### Component 7: Reference Library

##### [NEW] references/owasp-llm-top10.md
New reference (not in either framework):
- LLM01: Prompt Injection — mitigations for code agents
- LLM02: Insecure Output Handling — output validation gates
- LLM03: Training Data Poisoning — awareness for fine-tuning
- LLM04: Model Denial of Service — context window limits
- LLM05: Supply Chain Vulnerabilities — skill supply chain
- LLM06: Sensitive Information Disclosure — secrets handling
- LLM07: Insecure Plugin Design — skill permission manifests
- LLM08: Excessive Agency — least-privilege constraints
- LLM09: Overreliance — verification-before-completion
- LLM10: Model Theft — API key management

##### [NEW] references/owasp-agentic-top10.md
New reference covering ASI01-ASI10 mapped to CodeHands skills.

Enhanced versions of: `testing-patterns.md`, `testing-anti-patterns.md`, `security-checklist.md`, `performance-checklist.md`, `accessibility-checklist.md`

---

### Phase 4: Marketplace & Audit Infrastructure (Weeks 9-12)

---

#### Component 8: Slash Commands

##### [NEW] commands/*.md (8 commands)
Map development phases to skill activations:
- `/spec` → brainstorming + spec-driven-development
- `/plan` → planning-and-task-breakdown + context-engineering
- `/build` → TDD + incremental-implementation + source-driven-development
- `/test` → verification-before-completion + browser-testing-with-devtools
- `/review` → code-review-and-quality + security-and-hardening + performance-optimization
- `/ship` → git-workflow + ci-cd + shipping-and-launch
- `/audit` → audit-and-reporting (generates dashboards)
- `/debug` → systematic-debugging

---

#### Component 9: Audit & Reporting Skill

##### [NEW] skills/audit-and-reporting/SKILL.md
The audit skill generates structured reports covering:
1. **Session Activity Report** — Which skills fired, when, outcomes, human decisions
2. **Code Quality Dashboard** — Test coverage trends, security finding counts, review feedback
3. **Model Usage Report** — Which models were used, token costs, model variant selections
4. **Compliance Report** — OWASP checklist completion, security review coverage

**Output formats:**
- `markdown` — Inline in terminal (default)
- `json` — Machine-readable for CI pipelines
- `html` — Self-contained HTML dashboard file

---

#### Component 10: Trusted Publishing Infrastructure

##### [NEW] docs/trusted-publishing.md
Guide for skill authors on setting up OIDC-based Trusted Publishing:
1. Configure GitHub Actions / GitLab CI workflow
2. Create trust relationship with CodeHands marketplace
3. Publish skill with automatic provenance attestations
4. Verification badge display

---

### Phase 5: Testing & Quality Assurance (Weeks 13-14)

---

#### Component 11: Skill Testing Framework

##### [NEW] tests/skill-format.test.sh
Validates every SKILL.md:
- YAML frontmatter is valid and contains all required fields
- All standard sections present (Overview, When to Use, Process, Rationalizations, Red Flags, Verification)
- `model_variants` section is syntactically valid
- `permissions` array uses valid capability identifiers
- Cross-references (`codehands:*`) resolve to existing skills
- Token count < 2,000 per SKILL.md

##### [NEW] tests/cross-reference.test.sh
Validates all `codehands:` references resolve to actual skill directories.

##### [NEW] tests/frontmatter.test.sh
Validates YAML frontmatter schema for all skills.

---

### Phase 6: Documentation & Launch (Weeks 15-16)

---

#### Component 12: Documentation

##### [NEW] docs/setup/*.md (8 guides)
Per-platform setup guides for: Claude Code, Cursor, Gemini CLI, Copilot, Codex, OpenCode, Windsurf, Kiro.

##### [NEW] docs/migration/from-superpowers.md
Migration guide: `superpowers:` → `codehands:` namespace translation, skill equivalencies, behavior differences.

##### [NEW] docs/migration/from-agent-skills.md
Migration guide: `agent-skills` → `codehands:` namespace translation.

##### [NEW] docs/platform-compatibility.md
Verified compatibility matrix (from Analysis_Results.md Appendix B) as a standalone reference.

---

## Open Questions

> [!IMPORTANT]
> **Scope**: Should we deliver all 33 skills in the first release, or ship a "core 16" MVP (P0 skills only) and iterate?

> [!IMPORTANT]
> **Skill Enhancement Depth**: For each skill rewrite, should we target ~100 lines (concise, token-efficient) or allow up to ~300 lines (comprehensive, with more examples)? The tradeoff is token cost vs. behavioral completeness.

> [!WARNING]
> **HTML Dashboard**: The audit-and-reporting skill proposes generating self-contained HTML dashboards. This requires the agent to produce HTML/CSS/JS — should we provide a pre-built template, or let the agent generate it dynamically?

---

## Verification Plan

### Automated Tests

1. **Skill Format Validation**
   ```bash
   bash tests/skill-format.test.sh    # All 33 skills pass format validation
   bash tests/frontmatter.test.sh     # All frontmatter schemas valid
   bash tests/cross-reference.test.sh # All codehands: references resolve
   ```

2. **Platform Adapter Verification**
   - Install CodeHands on each of the 8 platforms
   - Verify SKILL.md loading
   - Test model detection hook
   - Test at least one slash command

3. **Token Budget Verification**
   ```bash
   bash scripts/skill-audit.sh --token-count  # All skills < 2,000 tokens
   ```

### Manual Verification

1. **End-to-end workflow test**: Run a full Define → Plan → Build → Verify → Review → Ship cycle on a sample project using CodeHands on Claude Code
2. **Cross-platform parity**: Same workflow tested on Cursor and Gemini CLI
3. **Security audit**: Run the security-auditor persona against CodeHands' own generated code
4. **Community review**: Publish draft to a small group of beta testers from both Superpowers and Agent-skills communities
