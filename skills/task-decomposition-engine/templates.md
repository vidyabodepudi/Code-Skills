# Task Decomposition Engine — Supporting Templates

## Session State Template

Save to `.codehands/session-state.md` and update throughout the session:

```markdown
## Codebase Profile — [Project Name]
Reconnoitered: YYYY-MM-DD HH:MM

### Identity
- Language: [e.g., TypeScript 5.3]
- Framework: [e.g., Next.js 14 App Router]
- Purpose: [one sentence]

### Testing
- Framework: [e.g., Jest + RTL]
- Command: `npm test`
- Count: ~[N] test files

### Build & Run
- Build: `npm run build`  Dev: `npm run dev`  Lint: `npm run lint`

### Patterns
- Naming: [camelCase vars, PascalCase components]
- Errors: [Custom AppError, try/catch at boundaries]
- Imports: [absolute paths via @/ alias]

---

## Session Journal — YYYY-MM-DD

### Request
[Original user request]

### Classification
[Category] — [Scope] / [Risk] / [Uncertainty]

### Skill Chain
1. [Skill 1]
2. [Skill 2]
...

### Decisions Made
- [Decision 1: chose X over Y because Z]

### Progress
- [x] Task 1: [description] (commit abc123)
- [/] Task 2: [description] (in progress)
- [ ] Task 3: [description]

### Discoveries
- [Finding that affects remaining work]

### Next Session
- Resume from Task N
- Consider: [open question]
```

## Skill Chain Templates

### Feature (Full Path)
```
task-decomposition-engine → codebase-reconnaissance →
brainstorming → spec-driven-development →
planning-and-task-breakdown → using-git-worktrees →
executing-plans [TDD + incremental per task] →
verification-before-completion → code-review-and-quality →
security-and-hardening → finishing-a-development-branch
```

### Feature (Lightweight — Isolated, Low Risk)
```
task-decomposition-engine → codebase-reconnaissance →
planning-and-task-breakdown →
executing-plans [TDD + incremental per task] →
verification-before-completion → finishing-a-development-branch
```

### Bug Fix
```
task-decomposition-engine → systematic-debugging →
test-driven-development (Prove-It) →
verification-before-completion
```

### Trivial Fix
```
task-decomposition-engine (fast path) →
test-driven-development → verify → commit
```

### Refactor
```
task-decomposition-engine → codebase-reconnaissance →
source-driven-development → code-simplification →
verification-before-completion
```

### Investigation
```
task-decomposition-engine → codebase-reconnaissance →
targeted analysis → report findings
```
