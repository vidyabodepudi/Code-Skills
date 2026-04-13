# /debug — Systematic Debugging

Activate the systematic debugging workflow.

## Skills Activated
1. `codehands:systematic-debugging` — 4-phase root cause investigation

## Process
1. **Phase 1: Root Cause Investigation** — Reproduce, gather evidence, trace execution, find divergence
2. **Phase 2: Pattern Analysis** — Check related symptoms, recent changes, false assumptions
3. **Phase 3: Hypothesis & Testing** — Form hypothesis, design verification, confirm
4. **Phase 4: Implementation** — Prove-It Pattern: reproduction test → fix → verify → regression check

## Rules
- Do NOT write any fix code until Phase 1 is complete
- If 3+ fix attempts fail, STOP and question the architecture
- Every fix requires a reproduction test (Prove-It Pattern)
