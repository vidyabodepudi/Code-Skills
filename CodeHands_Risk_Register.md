# CodeHands — Risk Register

> Last Updated: 2026-04-12
> Status: Active
> Owner: CodeHands Core Team

---

## Risk Matrix Legend

| Likelihood \ Impact | Low | Medium | High | Critical |
|---|---|---|---|---|
| **High** | 🟡 Monitor | 🟠 Mitigate | 🔴 Urgent | 🔴 Urgent |
| **Medium** | 🟢 Accept | 🟡 Monitor | 🟠 Mitigate | 🔴 Urgent |
| **Low** | 🟢 Accept | 🟢 Accept | 🟡 Monitor | 🟠 Mitigate |

---

## Active Risks

### R1: Community Fragmentation 🟠

| Field | Detail |
|---|---|
| **Description** | Superpowers/Agent-skills communities may resist CodeHands as a "replacement" rather than an evolution, leading to fork wars or fragmented adoption |
| **Impact** | High |
| **Likelihood** | Medium |
| **Category** | Community / Adoption |
| **Owner** | Community Lead |
| **Mitigation** | 1. Position CodeHands as a "from-the-ground-up" project inspired by both — not a hostile fork<br>2. Credit both projects prominently (Jesse Vincent / obra, Addy Osmani)<br>3. Apache 2.0 license signals openness and commercial-friendliness<br>4. Community governance (not maintainer-dictated) ensures all voices are heard<br>5. Open RFC process for major decisions |
| **Contingency** | If fragmentation occurs, focus on skill-level compatibility so users can run CodeHands skills alongside either framework |
| **Status** | Open |

---

### R2: Token Bloat 🟠

| Field | Detail |
|---|---|
| **Description** | Combined 33+ skills exceed agent context windows, degrading model performance and increasing costs |
| **Impact** | High |
| **Likelihood** | Medium |
| **Category** | Technical / Performance |
| **Owner** | Skill Engine Lead |
| **Mitigation** | 1. Strict progressive disclosure: SKILL.md < 2,000 tokens, references loaded on demand<br>2. Total framework default footprint < 5,000 tokens<br>3. Only description injected into system prompt — full skill loaded only when triggered<br>4. Token budget tracking in audit logs<br>5. Context engineering skill teaches agents to manage their own context |
| **Contingency** | If token budgets are still exceeded, implement tiered skill loading (core-only, extended, full) |
| **Status** | Open |

---

### R3: Platform Fragmentation 🟡

| Field | Detail |
|---|---|
| **Description** | Agent runtimes (Claude Code, Cursor, Gemini CLI, etc.) evolve instruction formats incompatibly, breaking platform adapters |
| **Impact** | Medium |
| **Likelihood** | High |
| **Category** | Technical / Compatibility |
| **Owner** | Platform Adapter Lead |
| **Mitigation** | 1. Abstract platform differences into slim adapter layer (CLAUDE.md, GEMINI.md, AGENTS.md, etc.)<br>2. Maintain verified compatibility matrix (see Analysis_Results.md Appendix B)<br>3. MCP + ACP convergence under Linux Foundation reduces fragmentation risk<br>4. SKILL.md format is plain markdown — maximally portable<br>5. Automated CI tests against each platform adapter |
| **Contingency** | If a platform diverges significantly, provide a migration guide and mark it as "degraded support" |
| **Status** | Open |

---

### R4: Skill Quality Dilution 🟡

| Field | Detail |
|---|---|
| **Description** | Open marketplace attracts low-quality or harmful skill submissions, eroding trust |
| **Impact** | Medium |
| **Likelihood** | High |
| **Category** | Ecosystem / Trust |
| **Owner** | Marketplace Lead |
| **Mitigation** | 1. OIDC-based Trusted Publishing with provenance attestations<br>2. Automated quality checks: frontmatter validation, required sections, anti-rationalization table presence<br>3. Community voting and rating system<br>4. Core skills (codehands-core) vs. community skills (codehands-community) distinction<br>5. Certification tiers: ✅ Verified, 🟡 Community, ⚠️ Unverified |
| **Contingency** | If quality issues persist, implement a mandatory review process for new skill submissions |
| **Status** | Open |

---

### R5: Security Theater 🟠

| Field | Detail |
|---|---|
| **Description** | Permission manifests exist in SKILL.md frontmatter but most platforms cannot enforce them at runtime (prompt-based only, except Codex sandboxing) |
| **Impact** | High |
| **Likelihood** | Medium |
| **Category** | Security / Governance |
| **Owner** | Security Lead |
| **Mitigation** | 1. Clearly document the enforcement gap in platform compatibility matrix<br>2. Permission manifests serve as "declaration of intent" even without runtime enforcement<br>3. Advocate to platform vendors for native permission APIs<br>4. Provide `codehands audit` CLI tool to scan skill permissions vs. actual behavior<br>5. Human-in-the-loop gates for destructive operations regardless of permission manifest |
| **Contingency** | If platforms never support enforcement, build a lightweight pre-flight checker that validates skill permissions before activation |
| **Status** | Open |

---

### R6: Maintenance Burden 🟡

| Field | Detail |
|---|---|
| **Description** | 33+ skills require constant updates as engineering practices, OWASP standards, and platform APIs evolve |
| **Impact** | Medium |
| **Likelihood** | High |
| **Category** | Operational / Sustainability |
| **Owner** | Core Maintainers |
| **Mitigation** | 1. Assign skill owners (1-2 maintainers per skill)<br>2. Automated staleness detection: skills not updated in 6 months flagged for review<br>3. Community contribution pipeline with clear standards (CONTRIBUTING.md)<br>4. Modular design: skills are independent — one stale skill doesn't affect others<br>5. Reference checklists (OWASP, WCAG, etc.) update independently of skill logic |
| **Contingency** | If maintenance becomes unsustainable, archive low-usage skills and focus on top-10 most-used |
| **Status** | Open |

---

### R7: Model Detection Reliability 🟡

| Field | Detail |
|---|---|
| **Description** | Deterministic model detection hooks may fail if platforms change their config file formats or API metadata |
| **Impact** | Low |
| **Likelihood** | Medium |
| **Category** | Technical / Model Detection |
| **Owner** | Platform Adapter Lead |
| **Mitigation** | 1. Multi-signal detection: platform config files → API metadata → platform instruction file name (CLAUDE.md vs GEMINI.md)<br>2. Graceful fallback: if model cannot be detected, use generic skill variant (no model_variants applied)<br>3. User override: allow manual model declaration in `.codehands/config.yml`<br>4. Never rely on asking the model "what are you?" (unreliable) |
| **Contingency** | If detection fails, default to the most conservative variant (most verbose instructions) |
| **Status** | Open |

---

### R8: Naming / Trademark Conflicts 🟢

| Field | Detail |
|---|---|
| **Description** | "CodeHands" name may conflict with existing trademarks or projects |
| **Impact** | Medium |
| **Likelihood** | Low |
| **Category** | Legal / Branding |
| **Owner** | Project Lead |
| **Mitigation** | 1. Conduct trademark search before public announcement<br>2. Register domain (codehands.dev or similar)<br>3. Reserve GitHub org and npm scope early |
| **Contingency** | If a conflict is discovered, rebrand early (before v1.0) |
| **Status** | Open |

---

### R9: Supply Chain Attack via Skill 🟠

| Field | Detail |
|---|---|
| **Description** | A malicious skill could instruct agents to exfiltrate secrets, introduce backdoors, or execute destructive commands |
| **Impact** | Critical |
| **Likelihood** | Low |
| **Category** | Security / Supply Chain |
| **Owner** | Security Lead |
| **Mitigation** | 1. OIDC Trusted Publishing links skill to source repository — tamper-evident<br>2. Provenance attestations prove build origin<br>3. Permission manifests make capabilities visible<br>4. Human-in-the-loop for destructive operations<br>5. Skill review queue for new submissions to the marketplace<br>6. skill-supply-chain-integrity skill educates agents about verification |
| **Contingency** | If a malicious skill is discovered, immediate revocation from marketplace + incident postmortem published |
| **Status** | Open |

---

## Risk Summary Dashboard

| Risk | Rating | Status | Trend |
|---|---|---|---|
| R1: Community Fragmentation | 🟠 Mitigate | Open | → Stable |
| R2: Token Bloat | 🟠 Mitigate | Open | → Stable |
| R3: Platform Fragmentation | 🟡 Monitor | Open | ↗ Increasing (more platforms emerging) |
| R4: Skill Quality Dilution | 🟡 Monitor | Open | → Stable |
| R5: Security Theater | 🟠 Mitigate | Open | ↘ Decreasing (MCP+ACP convergence) |
| R6: Maintenance Burden | 🟡 Monitor | Open | → Stable |
| R7: Model Detection Reliability | 🟡 Monitor | Open | → Stable |
| R8: Naming / Trademark | 🟢 Accept | Open | → Stable |
| R9: Supply Chain Attack | 🟠 Mitigate | Open | → Stable |
