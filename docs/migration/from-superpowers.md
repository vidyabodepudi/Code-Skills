# Migrating from Superpowers to CodeHands

This guide helps users of [`obra/superpowers`](https://github.com/obra/superpowers) transition to CodeHands.

## Why Migrate?

CodeHands is a from-the-ground-up reimplementation that preserves Superpowers' orchestration strengths while adding:
- Comprehensive security coverage (OWASP Top 10 + Agentic Security Top 10)
- Frontend, API, and performance skills
- Formal testing framework with format validation
- 8-platform compatibility (vs. single-platform)
- Model-specific enforcement via `model_variants`
- Community governance under Apache 2.0

## Namespace Migration

Replace `superpowers:` references with `codehands:`:

```diff
-Read and follow the superpowers:test-first skill
+Read and follow the codehands:test-driven-development skill
```

## Skill Mapping

| Superpowers Skill | CodeHands Equivalent | What Changed |
|---|---|---|
| `test-first` | `test-driven-development` | Added test pyramid, DAMP, Prove-It Pattern, Beyonce Rule |
| `systematic-debugging` | `systematic-debugging` | Added 3-fix escalation rule, pattern analysis phase |
| `create-a-spec` | `spec-driven-development` | Added RFC 2119 keywords, testable acceptance criteria |
| `brainstorming` | `brainstorming` | Added visual companion, streamlined to 5 steps |
| `plan-and-break-down-tasks` | `planning-and-task-breakdown` | Added size estimation, dependency ordering |
| `incremental-implementation` | `incremental-implementation` | Added one-behavior-per-increment rule |
| `code-review` | `code-review-and-quality` | Added two-stage pipeline with severity classification |
| `source-driven-development` | `source-driven-development` | Added targeted improvement exception |
| `verification-before-completion` | `verification-before-completion` | Added evidence format requirement |
| `dispatch-work-to-subagents` | `subagent-driven-development` | Added focused context scoping, prompt template |
| `dispatch-parallel-subagents` | `dispatching-parallel-agents` | Added dispatch log format, conflict resolution |
| `executing-plans` | `executing-plans` | Added session continuity guidance |
| `using-git-worktrees` | `using-git-worktrees` | Added baseline verification step |
| `finish-dev-branch` | `finishing-a-development-branch` | Added cleanup checklist, presentation options |

## New Skills (Not in Superpowers)

CodeHands adds 19 skills that Superpowers didn't have:

- `security-and-hardening` — OWASP Top 10 with code examples
- `context-engineering` — Context hierarchy management
- `frontend-ui-engineering` — Component architecture, accessibility
- `api-and-interface-design` — REST conventions, pagination
- `browser-testing-with-devtools` — DevTools MCP verification
- `performance-optimization` — Measure-first methodology
- `code-simplification` — Targeted refactoring
- `ci-cd-and-automation` — Pipeline design
- `shipping-and-launch` — Pre-launch checklists
- `documentation-and-adrs` — ADR format
- `deprecation-and-migration` — Safe deprecation
- `git-workflow-and-versioning` — Conventional commits, semver
- `agent-security-posture` — Agent behavior boundaries
- `skill-supply-chain-integrity` — Skill verification
- `audit-and-compliance` — License and regulatory
- `audit-and-reporting` — Dashboards and reports
- `writing-skills` — Skill authoring guide
- `using-codehands` — Framework onboarding

## Setup

1. Copy the appropriate adapter to your project root:
   ```bash
   # For Claude Code
   cp adapters/CLAUDE.md your-project/CLAUDE.md
   
   # For Gemini CLI
   cp adapters/GEMINI.md your-project/GEMINI.md
   ```

2. Copy the skills directory:
   ```bash
   cp -r skills/ your-project/.codehands/skills/
   ```

3. Remove old Superpowers references from your project.
