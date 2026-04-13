# /review — Review Phase

Activate the Review phase of the CodeHands lifecycle.

## Skills Activated
1. `codehands:code-review-and-quality` — Two-stage review: spec compliance then quality
2. `codehands:security-and-hardening` — OWASP Top 10 security review
3. `codehands:performance-optimization` — Performance review (if applicable)
4. `codehands:code-simplification` — Complexity reduction (if applicable)

## Process
1. **Stage 1: Spec compliance** — Read the spec, verify all requirements implemented
2. **Stage 2: Code quality** — Correctness, tests, readability, consistency, simplicity
3. **Security review** — Run through security checklist (`references/security-checklist.md`)
4. **Performance review** — Check for obvious issues (`references/performance-checklist.md`)
5. Produce review output with severity-classified findings

## Agent Personas Available
- `agents/code-reviewer.md` — For general code review
- `agents/security-auditor.md` — For security-focused review
- `agents/performance-engineer.md` — For performance-focused review
