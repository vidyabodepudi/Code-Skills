# CodeHands Skill Anatomy Specification v2

> This is the canonical specification for all CodeHands skills. Every skill MUST conform to this specification.

## Directory Structure

Every skill lives in its own directory under `skills/`:
```
skills/
  skill-name/
    SKILL.md              # Required: The skill definition
    supporting-file.md    # Optional: Reference material loaded on demand
```

## Frontmatter (Required)

```yaml
---
# ── REQUIRED ──────────────────────────────────────────────────
name: skill-name-with-hyphens
description: >
  What this skill does (third person). Use when [trigger conditions].
  Maximum 1024 characters. Do NOT summarize the process — if the
  description contains steps, agents may follow the summary instead
  of reading the full skill.
version: 1.0.0

# ── RECOMMENDED ───────────────────────────────────────────────
permissions:
  - fs.read          # Read project files
  - fs.write         # Write/modify project files
  - shell.exec       # Execute shell commands
  - net.http         # Make HTTP requests
  - browser.inspect  # Read browser state via DevTools MCP

triggers:
  - context: "implementing any feature"    # Contextual auto-activation
  - context: "fixing a bug"
  - command: "/build"                       # Slash command activation
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

# ── MODEL-SPECIFIC TUNING ────────────────────────────────────
# Loaded dynamically via deterministic model detection hooks.
# See docs/model-detection.md for how the active model is detected.
model_variants:
  claude:
    enforcement_style: "human-partner collaborative"
    # Claude responds well to collaborative framing: "your human partner"
  gemini:
    enforcement_style: "structured-checklist"
    # Gemini responds well to numbered checklists and explicit gates
  gpt:
    enforcement_style: "directive-imperative"
    # GPT responds well to direct imperatives and MUST/SHALL language

# ── PUBLISHING ────────────────────────────────────────────────
author: codehands-core          # or github-username for community skills
signed: true                    # OIDC Trusted Publishing attestation
---
```

### Frontmatter Rules

- `name`: Lowercase, hyphen-separated. MUST match the directory name.
- `description`: Starts with what the skill does (third person), followed by trigger conditions. Include both *what* and *when*. Maximum 1024 characters.
- `version`: Semantic versioning (major.minor.patch).
- `permissions`: Array of capability identifiers the skill requires. Valid values: `fs.read`, `fs.write`, `shell.exec`, `net.http`, `browser.inspect`, `secrets.read`.
- `triggers`: Array of activation conditions. `context` triggers activate based on agent activity. `command` triggers activate via slash commands.
- `platforms`: Array of compatible agent runtimes. Omit if compatible with all platforms.
- `dependencies`: Array of other CodeHands skills required, with semver ranges.
- `model_variants`: Model-specific behavioral tuning. Keys match the output of the model detection hook.
- `author`: `codehands-core` for official skills, GitHub username for community skills.
- `signed`: Whether the skill has OIDC Trusted Publishing attestation.

**Why descriptions matter:** Agents discover skills by reading descriptions. The description is injected into the system prompt, so it must tell the agent both what the skill provides and when to activate it. Never summarize the workflow in the description.

## Standard Sections

```markdown
## Overview
One-two sentences explaining what this skill does and why it matters.
The "elevator pitch" — should answer: What does this skill do,
and why should an agent follow it?

## When to Use
- Positive triggers: "Use when X"
- Negative exclusions: "NOT for Y"
Helps agents decide if this skill applies to the current task.

## Process
The step-by-step workflow the agent follows. This is the heart of the skill.
Must be specific and actionable — not vague advice.

GOOD: "Run `npm test` and verify all tests pass before proceeding"
BAD:  "Make sure the tests work"

Include:
- Numbered steps or named phases
- Code examples where they help
- Decision trees (ASCII or mermaid) at decision points
- Verification gates between phases

## Common Rationalizations
| Rationalization | Reality |
|---|---|
| "This is too simple for X" | [Why the excuse is wrong] |
| "I'll do X later" | [Why now, not later] |

The most critical section. These are excuses agents use to skip steps,
paired with factual rebuttals. This is BEHAVIORAL SHAPING CODE, not
optional documentation. Every skip-worthy step needs an entry here.

## Red Flags
- Observable signs that the skill is being violated
- Patterns to watch for during code review and self-monitoring
- Things that indicate the agent has rationalized past a gate

## Verification
After completing the skill's process, confirm:
- [ ] Exit criterion with evidence requirement (test output, screenshot, etc.)
- [ ] Every checkbox MUST be verifiable with evidence

## See Also
- Cross-references to related CodeHands skills: `codehands:skill-name`
- References to checklists: `references/security-checklist.md`
```

## Supporting Files

Create supporting files only when:
- Reference material exceeds 100 lines (keep SKILL.md focused)
- Code tools, scripts, or templates are needed
- Checklists are long enough to justify separate files

Keep patterns and principles inline when under 50 lines.

## Writing Principles

1. **Process over knowledge.** Skills are workflows, not reference docs. Steps, not facts.
2. **Specific over general.** `Run npm test` beats `verify the tests`.
3. **Evidence over assumption.** Every verification checkbox requires proof.
4. **Anti-rationalization as first-class.** Every skip-worthy step needs a counter-argument.
5. **Progressive disclosure.** SKILL.md is the entry point. Supporting files load on demand.
6. **Token-conscious.** Every section must justify its inclusion. If removing it wouldn't change agent behavior, remove it. Target < 2,000 tokens per SKILL.md.

## Naming Conventions

- Skill directories: `lowercase-hyphen-separated`
- Skill files: `SKILL.md` (always uppercase)
- Supporting files: `lowercase-hyphen-separated.md`
- References: stored in `references/` at the project root, not inside skill directories
- Cross-references: `codehands:<skill-name>`

## Cross-Skill References

Reference other skills by namespaced name:
```markdown
Follow the `codehands:test-driven-development` skill for writing tests.
If the build breaks, use the `codehands:systematic-debugging` skill.
```

Don't duplicate content between skills — reference and link instead.

## Token Budget

| Component | Budget |
|---|---|
| SKILL.md body | < 2,000 tokens |
| Description (in system prompt) | < 200 tokens |
| Supporting files | Loaded on demand (zero overhead when unused) |
| Total framework default footprint | < 5,000 tokens |
