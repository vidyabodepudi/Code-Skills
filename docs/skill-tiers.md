# CodeHands Skill Tier System

## Overview

Skills are organized into three tiers that control when they are loaded into context. This reduces token consumption per session by ~70% compared to loading all skills.

## Tiers

### Tier 0 — Always On (~8,000 tokens)
These skills are loaded at the start of EVERY session automatically. They represent the non-negotiable engineering foundation.

| Skill | Tokens | Why Always-On |
|---|---|---|
| `task-decomposition-engine` | ~2,242 | Root orchestrator — routes every request |
| `test-driven-development` | ~2,242 | Iron Law TDD — applies to all code |
| `source-driven-development` | ~1,061 | Read-before-write — prevents clobbering |
| `verification-before-completion` | ~1,054 | Nothing ships without evidence |
| `codebase-reconnaissance` | ~1,560 | Understanding before modifying |

### Tier 1 — Phase-Activated (~12,000 tokens available)
Loaded when the task-decomposition-engine routes to a specific phase. Only the relevant phase's skills load.

| Skill | Tokens | Activated By |
|---|---|---|
| `brainstorming` | ~1,807 | /spec, Feature classification |
| `spec-driven-development` | ~1,405 | /spec, Feature classification |
| `planning-and-task-breakdown` | ~1,746 | /plan, Feature/Refactor classification |
| `context-engineering` | ~1,995 | /plan, high-uncertainty assessment |
| `incremental-implementation` | ~943 | /build |
| `systematic-debugging` | ~1,886 | /debug, Bug classification |
| `code-review-and-quality` | ~1,299 | /review |
| `security-and-hardening` | ~2,477 | /review, High/Critical risk |
| `finishing-a-development-branch` | ~866 | /ship |
| `git-workflow-and-versioning` | ~1,035 | /ship |
| `risk-assessment-and-escalation` | ~1,342 | Medium+ risk assessment |
| `agent-security` | ~1,054 | /audit, session start |
| `ci-cd-and-shipping` | ~959 | /ship |
| `code-simplification` | ~941 | /review, Refactor classification |
| `executing-plans` | ~948 | /build, multi-task plans |

### Tier 2 — Specialist (~10,000 tokens available)
Loaded ONLY when explicitly needed by the task or detected by the engine.

| Skill | Tokens | Loaded When |
|---|---|---|
| `frontend-ui-engineering` | ~1,198 | Frontend/UI work detected |
| `api-and-interface-design` | ~1,108 | API design/endpoint work detected |
| `browser-testing-with-devtools` | ~867 | UI verification needed |
| `performance-optimization` | ~1,034 | Performance issues or /review |
| `multi-agent-orchestration` | ~1,030 | 3+ parallelizable tasks |
| `using-git-worktrees` | ~852 | Isolated workspace needed |
| `deprecation-and-migration` | ~708 | Deprecating features |
| `documentation-and-adrs` | ~759 | /ship, architecture decisions |
| `audit-and-governance` | ~1,005 | /audit |
| `writing-skills` | ~1,263 | Authoring new skills |
| `using-codehands` | ~1,187 | User asks about CodeHands |

## Typical Session Context Costs

| Scenario | Skills Loaded | ~Tokens |
|---|---|---|
| **Quick bug fix** | T0 + systematic-debugging | ~9,900 |
| **New feature** | T0 + brainstorming + spec + planning + incremental + code-review | ~17,300 |
| **Refactor** | T0 + planning + code-simplification + code-review | ~12,000 |
| **Security audit** | T0 + security-and-hardening + agent-security + audit-and-governance | ~12,600 |
| **All skills loaded** | Everything | ~39,300 |

## How Tiers Work

1. **Session start** → Load Tier 0 skills automatically
2. **Engine classification** → Determine which phase(s) apply
3. **Phase activation** → Load relevant Tier 1 skills
4. **Specialist detection** → Load Tier 2 skills only when signals match
5. **Session end** → No skills persist (fresh load next session)

## Configuration

In `.codehands/config.yml`:

```yaml
tiers:
  # Override tier assignments for project-specific needs
  always_on:
    - security-and-hardening   # Promote to T0 for fintech
  never_load:
    - writing-skills           # Not needed in this project
```
