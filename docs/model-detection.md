# Model Detection — CodeHands Documentation

## How Model Detection Works

CodeHands detects which LLM is active to adjust skill enforcement language via `model_variants` in SKILL.md frontmatter. This is done **deterministically** — we NEVER ask the model "what are you?" because LLM self-identification is unreliable.

## Detection Hierarchy

```
1. CODEHANDS_MODEL env var  →  Highest priority (user override)
2. .codehands/config.yml    →  Project-level setting
3. Platform file detection  →  Automatic (lowest priority)
4. Fallback: "generic"      →  No model-specific tuning
```

### Signal 1: Environment Variable

```bash
export CODEHANDS_MODEL=claude
```

Overrides all other detection. Useful for CI/CD environments or when using non-standard setups.

### Signal 2: Config File

```yaml
# .codehands/config.yml
model: gemini
```

Project-level override. Useful when a project standardizes on one model.

### Signal 3: Platform File Presence

| File Detected | Model Identified |
|---|---|
| `CLAUDE.md` or `.claude/` | claude |
| `GEMINI.md` or `.gemini/` | gemini |
| `.github/copilot-instructions.md` | copilot |
| `.windsurfrules` | windsurf |
| `.kiro/` | kiro |
| `AGENTS.md` + `$CODEX_MODEL` env var | codex |
| `AGENTS.md` (no Codex env) | opencode |
| `.cursor/` or `.cursor-plugin/` | cursor |

### Signal 4: Fallback

If no signal matches, the model is `"generic"` — skill content is used as-is with no model-specific tuning.

## Enforcement Styles

Each detected model maps to an enforcement style:

| Model | Style | What It Means |
|---|---|---|
| **claude**, **cursor** | `human-partner collaborative` | Collaborative framing, "your human partner," explain reasoning |
| **gemini**, **kiro** | `structured-checklist` | Numbered checklists, explicit gates, phase markers |
| **gpt**, **codex**, **copilot**, **opencode** | `directive-imperative` | MUST/SHALL language, direct imperatives, no hedging |

## How Skills Use Model Variants

Skills include a `model_variants` block in frontmatter:

```yaml
model_variants:
  claude:
    enforcement_style: "human-partner collaborative"
    iron_law_phrasing: "Your human partner expects tests first."
  gemini:
    enforcement_style: "structured-checklist"
    iron_law_phrasing: "CHECKLIST: ☐ Write failing test ☐ Confirm RED"
  gpt:
    enforcement_style: "directive-imperative"
    iron_law_phrasing: "You MUST write a failing test. No exceptions."
```

When the model is detected, the platform adapter loads the matching variant and adjusts the skill's phrasing accordingly.

## Why Not Ask the Model?

LLMs cannot reliably self-identify:
- They may report whatever identity their system prompt says
- They may "hallucinate" their model name
- They may be fine-tuned versions with different names
- There is no standardized self-identification API

Deterministic detection from config files is always reliable.

## Running the Detection Script

```bash
bash scripts/model-detect.sh
```

Output:
```
CodeHands Model Detection
=========================
Detected model:     claude
Enforcement style:  human-partner collaborative

Override with: export CODEHANDS_MODEL=<model>
Or set in:     .codehands/config.yml → model: <model>
```

## Limitations

- If multiple platform config files exist (e.g., both CLAUDE.md and GEMINI.md), the first match in the detection order wins
- Model variants are advisory — they change phrasing, not behavior. The underlying skill rules are identical regardless of model.
- Custom or fine-tuned models should use the `CODEHANDS_MODEL` env var with the closest base model name
