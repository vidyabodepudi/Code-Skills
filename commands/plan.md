# /plan — Plan Phase

Activate the Plan phase of the CodeHands lifecycle.

## Skills Activated
1. `codehands:planning-and-task-breakdown` — Decompose the approved spec into bite-sized tasks with file paths, verification commands, and dependency ordering
2. `codehands:context-engineering` — Load context deliberately in hierarchy order

## Process
1. Read the approved spec completely
2. Identify implementation order (shared infra → data → logic → integration → presentation)
3. Write task definitions with: files, implementation scope, test expectations, verification command
4. Size each task (Small < 50 LOC, Medium 50-150, Large 150+)
5. Save plan to `docs/plans/YYYY-MM-DD-<topic>-plan.md`
6. Get user approval before proceeding to build

## Hard Gate
Do NOT start implementing until the plan is approved.
