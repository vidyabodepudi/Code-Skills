# Changelog

All notable changes to CodeHands will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-04-12

### Added

#### Core Framework
- Apache 2.0 license
- Project README with architecture overview and quick start
- Skill Anatomy Specification v2 with model_variants, permissions, and token budget
- Skill template for new authors
- Contributing guidelines

#### 33 Skills (All Written From Scratch)

**Define Phase:**
- `brainstorming` — Hard-gate design exploration with Socratic dialogue
- `spec-driven-development` — RFC 2119 requirements with testable acceptance criteria

**Plan Phase:**
- `planning-and-task-breakdown` — Bite-sized tasks with file paths and dependency ordering
- `context-engineering` — 5-level context hierarchy with confusion management

**Build Phase:**
- `test-driven-development` — Iron Law TDD with Prove-It Pattern and DAMP naming
- `incremental-implementation` — One-behavior-per-increment cycle
- `source-driven-development` — Read-before-write with pattern following
- `frontend-ui-engineering` — Component architecture, accessibility, responsive design
- `api-and-interface-design` — REST conventions, consistent responses, pagination
- `using-git-worktrees` — Isolated workspaces with baseline verification

**Orchestrate Phase:**
- `subagent-driven-development` — Multi-agent dispatch with two-stage review gates
- `dispatching-parallel-agents` — Parallel dispatch with conflict avoidance
- `executing-plans` — Plan execution coordination across sessions

**Verify Phase:**
- `systematic-debugging` — 4-phase root cause investigation with 3-fix escalation
- `browser-testing-with-devtools` — DevTools MCP visual verification
- `verification-before-completion` — Evidence-based completion gate

**Review Phase:**
- `code-review-and-quality` — Two-stage pipeline: spec compliance then quality
- `code-simplification` — Targeted refactoring with complexity signals
- `security-and-hardening` — OWASP Top 10 with concrete code examples
- `performance-optimization` — Measure-first methodology

**Ship Phase:**
- `finishing-a-development-branch` — Merge/PR/keep/discard decision flow
- `git-workflow-and-versioning` — Conventional commits, semver, feature flags
- `ci-cd-and-automation` — Pipeline stages and deployment strategies
- `deprecation-and-migration` — Safe deprecation with sunset timelines
- `documentation-and-adrs` — ADR format and documentation standards
- `shipping-and-launch` — Pre-launch, rollback, and post-deploy checklists

**Govern Phase:**
- `agent-security-posture` — OWASP LLM Top 10 and Agentic Security Top 10 posture
- `skill-supply-chain-integrity` — OIDC Trusted Publishing verification
- `audit-and-compliance` — License, data handling, and regulatory compliance
- `audit-and-reporting` — Structured reports and dashboards

**Meta:**
- `writing-skills` — Skill authoring guide with writing principles
- `using-codehands` — Framework onboarding and lifecycle overview

#### 5 Agent Personas
- `code-reviewer` — Two-stage review with severity classification
- `test-engineer` — Test pyramid, DAMP, anti-pattern detection
- `security-auditor` — OWASP-aligned with PoC requirements
- `performance-engineer` — Core Web Vitals, profiling, database optimization (NEW)
- `architect` — Design review, SOLID, component boundaries (NEW)

#### 7 Reference Documents
- `owasp-llm-top10.md` — Mapped to CodeHands mitigations (NEW)
- `owasp-agentic-top10.md` — ASI-01 to ASI-10 coverage (NEW)
- `security-checklist.md` — Quick-scan security checklist
- `testing-patterns.md` — Framework-specific patterns (JS, Python, Go, Rust)
- `testing-anti-patterns.md` — 10 anti-patterns with code examples
- `performance-checklist.md` — Frontend, backend, and algorithmic
- `accessibility-checklist.md` — WCAG 2.2 AA compliance

#### 8 Slash Commands
- `/spec` — Define phase (brainstorming + spec)
- `/plan` — Plan phase (task breakdown + context)
- `/build` — Build phase (TDD + incremental + worktrees)
- `/test` — Verify phase (verification + browser testing)
- `/review` — Review phase (code review + security + performance)
- `/ship` — Ship phase (git workflow + CI/CD + launch)
- `/debug` — Systematic debugging
- `/audit` — Audit and reporting

#### 7 Platform Adapters
- Claude Code (CLAUDE.md)
- Gemini CLI (GEMINI.md)
- Codex / OpenCode (AGENTS.md)
- Cursor (.cursor-plugin/manifest.json)
- GitHub Copilot (copilot-instructions.md)
- Windsurf (.windsurfrules)
- Kiro (kiro/manifest.json)

#### Model Detection
- Deterministic 4-signal detection (env var → config → platform file → fallback)
- Model-specific enforcement styles (collaborative, checklist, directive)
- model-detect.sh utility script

#### Testing Framework
- skill-format.test.sh — Format and section validation
- frontmatter.test.sh — YAML schema validation
- cross-reference.test.sh — codehands: namespace resolution

#### Documentation
- Skill Anatomy Specification (docs/skill-anatomy.md)
- Model Detection Guide (docs/model-detection.md)
- Analysis & PRD (Analysis_Results.md)
- Risk Register (CodeHands_Risk_Register.md)
- Implementation Plan (Implementation_Plan.md)
