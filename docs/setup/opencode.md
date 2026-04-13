# OpenCode — CodeHands Setup Guide

## Quick Start

```bash
cp adapters/AGENTS.md your-project/AGENTS.md
cp -r skills/ your-project/.codehands/skills/
cp -r agents/ your-project/.codehands/agents/
cp -r references/ your-project/.codehands/references/
cp -r commands/ your-project/.codehands/commands/
```

## How It Works

1. OpenCode reads `AGENTS.md` from your project root
2. Model detection: `AGENTS.md` without `$CODEX_MODEL` env → OpenCode detected
3. Enforcement style: **directive-imperative** (MUST/SHALL language)
