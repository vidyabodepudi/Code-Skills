# Migrating from Agent Skills to CodeHands

This guide helps users of [`addyosmani/agent-skills`](https://github.com/addyosmani/agent-skills) transition to CodeHands.

## Why Migrate?

CodeHands is a from-the-ground-up reimplementation that preserves Agent Skills' comprehensive SDLC coverage while adding:
- Structured anti-rationalization tables (behavioral enforcement)
- Multi-agent orchestration (subagent dispatch, parallel agents)
- Formal YAML frontmatter with permissions, model variants, and dependencies
- Testing framework with automated validation
- Agent personas for specialized reviews
- Community governance under Apache 2.0

## Namespace Migration

Agent Skills doesn't use a namespace. CodeHands uses `codehands:` for all cross-references:

```diff
-See the security hardening practices
+See `codehands:security-and-hardening`
```

## Skill Mapping

| Agent Skills Topic | CodeHands Equivalent | What Changed |
|---|---|---|
| Testing best practices | `test-driven-development` | Added Iron Law enforcement, Prove-It Pattern |
| Debugging approaches | `systematic-debugging` | Added 4-phase structure, 3-fix escalation |
| Security hardening | `security-and-hardening` | Added OWASP Top 10 mapping, agent-specific threats |
| Code review | `code-review-and-quality` | Added two-stage pipeline (spec compliance → quality) |
| Performance | `performance-optimization` | Added measure-first methodology |
| Accessibility | `references/accessibility-checklist.md` | Moved to reference (checklist format) |
| API design | `api-and-interface-design` | Added REST conventions, versioning, pagination |
| Documentation | `documentation-and-adrs` | Added ADR format |
| CI/CD | `ci-cd-and-automation` | Added pipeline stages, deployment strategies |
| Frontend | `frontend-ui-engineering` | Added component architecture, state management |
| Context management | `context-engineering` | Added 5-level hierarchy, confusion management |
| Planning | `planning-and-task-breakdown` | Added size estimation, dependency graphs |

## New Skills (Not in Agent Skills)

CodeHands adds these capabilities:

- `brainstorming` — Hard-gate design exploration before implementation
- `spec-driven-development` — RFC 2119 formal requirements
- `incremental-implementation` — One-behavior-per-increment cycle
- `source-driven-development` — Read-before-write enforcement
- `verification-before-completion` — Evidence-based completion gate
- `subagent-driven-development` — Multi-agent orchestration
- `dispatching-parallel-agents` — Parallel work dispatch
- `executing-plans` — Cross-session plan execution
- `using-git-worktrees` — Isolated development workspaces
- `finishing-a-development-branch` — Branch completion flow
- `agent-security-posture` — Agent behavioral boundaries
- `skill-supply-chain-integrity` — Skill provenance verification
- `audit-and-compliance` — Regulatory compliance
- `audit-and-reporting` — Dashboards and reports
- `writing-skills` — Skill authoring guide
- `using-codehands` — Framework onboarding

## Key Architectural Differences

| Aspect | Agent Skills | CodeHands |
|---|---|---|
| **Format** | Markdown with sections | YAML frontmatter + Markdown body |
| **Enforcement** | Advisory guidance | Anti-rationalization tables + hard gates |
| **Model awareness** | None | `model_variants` per skill |
| **Permissions** | Not declared | Declared in frontmatter |
| **Testing** | None | 3 automated validation scripts |
| **Personas** | Referenced | Formal persona files with output templates |
| **Platform support** | General guidance | 8 platform-specific adapters |

## Setup

1. Copy the appropriate adapter to your project root
2. Copy the skills directory
3. Remove old agent-skills references from your project configuration
