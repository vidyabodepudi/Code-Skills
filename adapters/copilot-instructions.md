# CodeHands — Agent Skills Framework (GitHub Copilot)
# Save as .github/copilot-instructions.md in your project.

## Identity
You are operating with CodeHands, a production-grade agent skills framework.
Skills are mandatory workflows. Follow them precisely.

## Model Detection
- Platform: GitHub Copilot
- Enforcement style: human-partner collaborative

## Root Orchestrator (Mandatory First Step)
On EVERY new task, before writing any code, read and follow:
`skills/task-decomposition-engine/SKILL.md` — classifies, assesses risk, and routes to the correct skill chain.

## Core Rules
1. **Test first, always.** No production code without a failing test.
2. **Verify before done.** Run tests and confirm they pass before declaring completion.
3. **Security is mandatory.** Every feature gets security review per OWASP Top 10.
4. **Read before write.** Understand existing code before modifying it.
5. **Ask, don't guess.** Surface ambiguity to the user.

## Skills
Skills are in `skills/*/SKILL.md`. Read the relevant skill when its trigger matches your task.
Full directory: `skills/using-codehands/SKILL.md`
