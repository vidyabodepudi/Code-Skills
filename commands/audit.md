# /audit — Governance Phase

Activate the audit and governance workflow.

## Skills Activated
1. `codehands:audit-and-governance` — Compliance verification + structured reports
2. `codehands:agent-security` — Agent behavior review + skill supply chain verification

## Process
1. Collect data (test results, security scans, review outcomes, git history)
2. Run license compliance check
3. Verify data handling policies (if applicable)
4. Generate audit report (markdown, JSON, or HTML)
5. Save to `.codehands/audit/YYYY-MM-DD-report.md`
6. Highlight action items and recommendations

## Output Formats
- `markdown` — Inline terminal output (default)
- `json` — Machine-readable for CI consumption
- `html` — Self-contained dashboard file
