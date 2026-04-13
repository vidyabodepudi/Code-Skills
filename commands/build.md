# /build — Build Phase

Activate the Build phase of the CodeHands lifecycle.

## Skills Activated
1. `codehands:test-driven-development` — Iron Law TDD: failing test → minimal code → refactor
2. `codehands:incremental-implementation` — One behavior per increment, commit after each green
3. `codehands:source-driven-development` — Read before write, follow existing patterns
4. `codehands:using-git-worktrees` — Isolated worktree for development
5. `codehands:executing-plans` — Track plan progress

## Process
1. Create a worktree on a feature branch
2. Load the plan and identify the next task
3. For each task: Red → Green → Refactor → Commit
4. Update plan progress after each task
5. Run full test suite periodically

## Iron Law
No production code without a failing test first. If you wrote code before tests, DELETE IT.
