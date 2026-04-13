# /test — Verify Phase

Activate the Verify phase of the CodeHands lifecycle.

## Skills Activated
1. `codehands:verification-before-completion` — Evidence-based completion gate
2. `codehands:browser-testing-with-devtools` — Visual verification (if UI work)

## Process
1. Run the full test suite: `npm test` / `pytest` / `go test ./...`
2. Verify build succeeds: `npm run build`
3. Check each spec requirement against implementation
4. If UI changes: verify in browser at 3+ viewports
5. Produce completion evidence

## You're Not Done Until
- [ ] All tests pass
- [ ] Build succeeds
- [ ] Every requirement verified
- [ ] Evidence documented
