# CodeHands — Agent Skills Framework
# Load this file as AGENTS.md for Codex / OpenCode integration.

# ─── IDENTITY ──────────────────────────────────────────────────────
# You are operating with CodeHands, a production-grade agent skills framework.
# Skills are mandatory workflows. Follow them precisely.

# ─── MODEL DETECTION ──────────────────────────────────────────────
# Platform: Codex / OpenCode
# Model: Detected via AGENTS.md presence + environment variables
# Enforcement style: directive-imperative
# Use MUST/SHALL language. Direct imperatives. No hedging.

# ─── ROOT ORCHESTRATOR (MANDATORY FIRST STEP) ────────────────────
# On EVERY new task, you MUST first follow:
#   skills/task-decomposition-engine/SKILL.md
# You MUST classify, assess, and route before writing any code.

# ─── MANDATORY RULES ─────────────────────────────────────────────
# You MUST write a failing test before any production code.
# You MUST run all tests before declaring any task complete.
# You MUST NOT start implementation without an approved design.
# You MUST NOT commit secrets or credentials to source code.
# You MUST NOT execute destructive operations without human approval.
# You MUST surface ambiguity instead of guessing.
# You MUST read the anti-rationalization table before skipping any skill step.

# ─── SKILL LOADING ────────────────────────────────────────────────
# Skills: skills/*/SKILL.md (auto-activate based on context)
# References: references/*.md (on demand)
# Personas: agents/*.md (for specialized reviews)
# Namespace: codehands:<skill-name>

# ─── LIFECYCLE ─────────────────────────────────────────────────────
# DEFINE → PLAN → BUILD → ORCHESTRATE → VERIFY → REVIEW → SHIP → GOVERN
# Full skill directory in skills/using-codehands/SKILL.md
