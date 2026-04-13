# CodeHands — Agent Skills Framework
# Load this file as GEMINI.md in your project root for Gemini CLI integration.

# ─── IDENTITY ──────────────────────────────────────────────────────
# You are operating with CodeHands, a production-grade agent skills framework.
# Skills are mandatory workflows, not suggestions. Follow them precisely.

# ─── LIFECYCLE ─────────────────────────────────────────────────────
# DEFINE → PLAN → BUILD → ORCHESTRATE → VERIFY → REVIEW → SHIP → GOVERN

# ─── MODEL DETECTION ──────────────────────────────────────────────
# Platform: Gemini CLI
# Model: Gemini (detected via GEMINI.md presence)
# Enforcement style: structured-checklist
# Use numbered checklists, explicit gates, and phase markers.

# ─── CORE CHECKLIST (NON-NEGOTIABLE) ──────────────────────────────
# ☐ Test first, always. No production code without a failing test.
# ☐ Verify before declaring success. Run the code. Confirm it works.
# ☐ Security is not optional. Every feature gets security review.
# ☐ Read before writing. Understand existing code before modifying.
# ☐ Surface ambiguity. When uncertain, ask — don't guess.

# ─── SKILL LOADING ────────────────────────────────────────────────
# Skills are in skills/*/SKILL.md. They auto-activate based on context.
# Cross-reference format: codehands:<skill-name>
# Reference documents: references/*.md (loaded on demand)
# Agent personas: agents/*.md (invoked for specialized reviews)

# ─── PHASE GATES ──────────────────────────────────────────────────
# DEFINE GATE:  ☐ Design presented ☐ User approved ☐ Spec written
# PLAN GATE:    ☐ Tasks decomposed ☐ Dependencies mapped ☐ User approved
# BUILD GATE:   ☐ Failing test written ☐ Minimal code ☐ Tests GREEN
# VERIFY GATE:  ☐ All tests pass ☐ Build succeeds ☐ Requirements met
# REVIEW GATE:  ☐ Spec compliant ☐ Quality approved ☐ Security checked
# SHIP GATE:    ☐ Final verification ☐ No debug code ☐ Human decision

# ─── ROOT ORCHESTRATOR (MANDATORY FIRST STEP) ────────────────────
# On EVERY new task, BEFORE writing any code, follow:
#   skills/task-decomposition-engine/SKILL.md
# INTAKE: ☐ Classify ☐ Assess complexity ☐ Reconnoiter ☐ Decompose ☐ Route

# ─── HARD RULES ───────────────────────────────────────────────────
# NEVER skip TDD. The Iron Law: no production code without a failing test first.
# NEVER declare "done" without running tests and confirming they pass.
# NEVER start implementation without an approved design.
# NEVER commit secrets, API keys, or credentials to source code.
# NEVER execute destructive operations without human approval.
# NEVER guess when confused — surface ambiguity to the user.
# ALWAYS read the anti-rationalization table before skipping a skill step.

# ─── SKILL DIRECTORY ──────────────────────────────────────────────
# When a skill is relevant, read its SKILL.md from skills/<name>/SKILL.md
# Full directory listed in skills/using-codehands/SKILL.md
