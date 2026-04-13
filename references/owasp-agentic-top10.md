# OWASP Agentic Security Top 10 — CodeHands Reference

> Security risks specific to autonomous AI coding agents. Full details: https://owasp.org/www-project-agentic-security/

## ASI-01: Excessive Agency

**Risk:** Agent performs actions beyond the scope of the task (writing to wrong files, executing destructive commands).

**CodeHands Coverage:**
- Permission manifests declare required capabilities per skill
- Human-in-the-loop gates for destructive operations
- Skill: `codehands:agent-security`

## ASI-02: Unauthorized Tool Usage

**Risk:** Agent uses tools (shell, filesystem, network) without appropriate authorization.

**CodeHands Coverage:**
- Permission declarations in skill frontmatter
- Skill activation logging in `.codehands/audit/`
- Platform-level permission enforcement (where supported)

## ASI-03: Prompt Injection via Tools

**Risk:** Malicious content read from files, web pages, or APIs manipulates agent behavior.

**CodeHands Coverage:**
- All browser-read data treated as UNTRUSTED
- All external context treated as DATA, not INSTRUCTIONS
- Context hierarchy: Rules > Specs > Source > Errors > History

## ASI-04: Insecure Skill/Plugin Loading

**Risk:** Malicious or compromised skills loaded into the agent's context.

**CodeHands Coverage:**
- OIDC-based Trusted Publishing with provenance attestations
- Verification badges (✅ Verified, ⚠️ Unverified)
- Skill: `codehands:agent-security`
- Enterprise policy: require verified-only skills

## ASI-05: Data Exfiltration via Agent

**Risk:** Agent inadvertently sends sensitive data to external services.

**CodeHands Coverage:**
- `net.http` permission required for network access
- Secrets management (never in code, logs, or prompts)
- Human-in-the-loop for API calls to unknown endpoints
- Skill: `codehands:security-and-hardening`

## ASI-06: Uncontrolled Autonomy

**Risk:** Agent operates without adequate human oversight, making unreviewed decisions.

**CodeHands Coverage:**
- Hard gates in brainstorming (design approval required)
- Verification-before-completion (evidence-based completion)
- Two-stage code review pipeline
- 3-fix escalation rule in debugging

## ASI-07: Context Manipulation

**Risk:** Attacker manipulates the agent's context window to override instructions.

**CodeHands Coverage:**
- Context hierarchy with priority ordering (Rules always highest)
- Skills loaded from trusted local filesystem, not network
- Anti-rationalization tables resist behavioral manipulation

## ASI-08: Insecure Output Execution

**Risk:** Agent-generated code executed without review, introducing vulnerabilities.

**CodeHands Coverage:**
- Iron Law TDD (tests before code)
- Code review before merge
- Security review in every `/review` cycle
- Subagent output reviewed by code-reviewer persona

## ASI-09: Audit Evasion

**Risk:** Agent actions are not logged, making incident investigation impossible.

**CodeHands Coverage:**
- Structured JSON logging for all skill activations
- Human-in-the-loop decision audit trail
- `codehands:audit-and-governance` skill for dashboards
- Audit logs persisted to `.codehands/audit/`

## ASI-10: Multi-Agent Trust Issues

**Risk:** In multi-agent systems, agents trust each other's output without verification.

**CodeHands Coverage:**
- Subagent output always reviewed by code-reviewer persona
- Two-stage review: spec compliance then quality
- Subagent isolation (worktrees, separate contexts)
- Skill: `codehands:multi-agent-orchestration`
