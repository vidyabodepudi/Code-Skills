# GitHub Copilot — CodeHands Setup Guide

## Quick Start

```bash
cp adapters/copilot-instructions.md your-project/.github/copilot-instructions.md
cp -r skills/ your-project/.codehands/skills/
cp -r agents/ your-project/.codehands/agents/
cp -r references/ your-project/.codehands/references/
cp -r commands/ your-project/.codehands/commands/
```

## How It Works

1. Copilot reads `.github/copilot-instructions.md` for project context
2. Skills are referenced as `codehands:<skill-name>` in conversations
3. Model detection: Copilot → "directive-imperative" enforcement style

## Limitations

- **No native slash commands** — Say "follow the /spec command" or reference `commands/spec.md`
- **No subagent dispatch** — Single-agent only
- **No browser DevTools** — Use component tests for UI verification
