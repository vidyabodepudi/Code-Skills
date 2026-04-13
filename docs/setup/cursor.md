# Cursor — CodeHands Setup Guide

## Quick Start

```bash
cp -r adapters/.cursor-plugin/ your-project/.cursor-plugin/
cp -r skills/ your-project/.codehands/skills/
cp -r agents/ your-project/.codehands/agents/
cp -r references/ your-project/.codehands/references/
cp -r commands/ your-project/.codehands/commands/
```

## How It Works

1. Cursor loads the `.cursor-plugin/manifest.json` configuration
2. Skills are available via the CodeHands skill directory
3. Model detection: Cursor → "human-partner collaborative" enforcement style
4. Background agents can be used for subagent dispatch

## Cursor-Specific Features

- **Background agents** — Use for `codehands:multi-agent-orchestration`
- **Inline editing** — Skills guide behavior during inline edits
- **Chat panel** — Slash commands work via "run the /spec command" phrasing
