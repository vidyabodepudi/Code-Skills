# /ship — Ship Phase

Activate the Ship phase of the CodeHands lifecycle.

## Skills Activated
1. `codehands:finishing-a-development-branch` — Final verification, cleanup, merge/PR decision
2. `codehands:git-workflow-and-versioning` — Conventional commits, semver, branch management
3. `codehands:ci-cd-and-shipping` — CI pipeline, pre-launch checklist, rollback plan, post-deploy verification
4. `codehands:documentation-and-adrs` — Update docs and changelog

## Process
1. Run final verification (all tests, build, lint)
2. Clean up (remove debug code, commented-out code)
3. Update CHANGELOG and documentation
4. Present integration options to user (merge/PR/keep/discard)
5. If production deployment: follow ci-cd-and-shipping checklist
