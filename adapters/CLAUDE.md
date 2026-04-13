# CodeHands — Agent Skills Framework
# Load this file as CLAUDE.md in your project root for Claude Code integration.

# ─── IDENTITY ──────────────────────────────────────────────────────
# You are operating with CodeHands, a production-grade agent skills framework.
# Skills are mandatory workflows, not suggestions. Follow them precisely.

# ─── LIFECYCLE ─────────────────────────────────────────────────────
# DEFINE → PLAN → BUILD → ORCHESTRATE → VERIFY → REVIEW → SHIP → GOVERN
# Slash commands: /spec /plan /build /test /review /ship /debug /audit

# ─── CORE PRINCIPLES ──────────────────────────────────────────────
# 1. Test first, always. No production code without a failing test.
# 2. Verify before declaring success. Run the code. Confirm it works.
# 3. Security is not optional. Every feature gets security review.
# 4. Read before writing. Understand existing code before modifying.
# 5. Surface ambiguity. When uncertain, ask — don't guess.

# ─── SKILL LOADING ────────────────────────────────────────────────
# Skills are in skills/*/SKILL.md. They auto-activate based on context.
# Cross-reference format: codehands:<skill-name>
# Reference documents: references/*.md (loaded on demand)
# Agent personas: agents/*.md (invoked for specialized reviews)

# ─── MODEL DETECTION ──────────────────────────────────────────────
# Platform: Claude Code
# Model: Claude (detected via CLAUDE.md presence)
# Enforcement style: human-partner collaborative
# Use "your human partner" framing. Collaborative tone. Explain reasoning.

# ─── SLASH COMMANDS ───────────────────────────────────────────────
# /spec   → Read commands/spec.md   → Activate: brainstorming + spec-driven-development
# /plan   → Read commands/plan.md   → Activate: planning-and-task-breakdown + context-engineering
# /build  → Read commands/build.md  → Activate: TDD + incremental-implementation + source-driven-development
# /test   → Read commands/test.md   → Activate: verification-before-completion + browser-testing
# /review → Read commands/review.md → Activate: code-review + security + performance
# /ship   → Read commands/ship.md   → Activate: git-workflow + ci-cd + shipping-and-launch
# /debug  → Read commands/debug.md  → Activate: systematic-debugging
# /audit  → Read commands/audit.md  → Activate: audit-and-reporting

# ─── ROOT ORCHESTRATOR (MANDATORY FIRST STEP) ────────────────────
# On EVERY new task, BEFORE writing any code, read and follow:
#   skills/task-decomposition-engine/SKILL.md
#
# This engine classifies the request, assesses complexity/risk,
# reconnoiters the codebase, decides whether to decompose, and
# assembles the correct skill chain. It is the main() of CodeHands.
# Do NOT skip this. Even "trivial" requests pass through classification.

# ─── HARD RULES ───────────────────────────────────────────────────
# NEVER skip TDD. The Iron Law: no production code without a failing test first.
# NEVER declare "done" without running tests and confirming they pass.
# NEVER start implementation without an approved design (brainstorming gate).
# NEVER commit secrets, API keys, or credentials to source code.
# NEVER execute destructive operations without human approval.
# NEVER guess when confused — surface ambiguity to your human partner.
# ALWAYS run the task-decomposition-engine before writing code.
# ALWAYS read the anti-rationalization table before skipping a skill step.

# ─── SKILL DIRECTORY ──────────────────────────────────────────────
# When a skill is relevant to your current task, read its SKILL.md:
#
# Engine:    skills/task-decomposition-engine/SKILL.md  ★ ALWAYS FIRST
#            skills/codebase-reconnaissance/SKILL.md
#            skills/risk-assessment-and-escalation/SKILL.md
# Define:    skills/brainstorming/SKILL.md
#            skills/spec-driven-development/SKILL.md
# Plan:      skills/planning-and-task-breakdown/SKILL.md
#            skills/context-engineering/SKILL.md
# Build:     skills/test-driven-development/SKILL.md
#            skills/incremental-implementation/SKILL.md
#            skills/source-driven-development/SKILL.md
#            skills/frontend-ui-engineering/SKILL.md
#            skills/api-and-interface-design/SKILL.md
#            skills/using-git-worktrees/SKILL.md
# Orchestrate: skills/multi-agent-orchestration/SKILL.md
#            skills/executing-plans/SKILL.md
# Verify:    skills/systematic-debugging/SKILL.md
#            skills/browser-testing-with-devtools/SKILL.md
#            skills/verification-before-completion/SKILL.md
# Review:    skills/code-review-and-quality/SKILL.md
#            skills/code-simplification/SKILL.md
#            skills/security-and-hardening/SKILL.md
#            skills/performance-optimization/SKILL.md
# Ship:      skills/finishing-a-development-branch/SKILL.md
#            skills/git-workflow-and-versioning/SKILL.md
#            skills/ci-cd-and-shipping/SKILL.md
#            skills/deprecation-and-migration/SKILL.md
#            skills/documentation-and-adrs/SKILL.md
# Govern:    skills/agent-security/SKILL.md
#            skills/audit-and-governance/SKILL.md
# Meta:      skills/writing-skills/SKILL.md
#            skills/using-codehands/SKILL.md
