# Kiro — CodeHands Setup Guide

## Quick Start

```bash
cp -r adapters/kiro/ your-project/.kiro/
cp -r skills/ your-project/.codehands/skills/
cp -r agents/ your-project/.codehands/agents/
cp -r references/ your-project/.codehands/references/
cp -r commands/ your-project/.codehands/commands/
```

## How It Works

1. Kiro reads `.kiro/manifest.json` from your project root
2. Model detection: `.kiro/` directory presence → Kiro detected
3. Enforcement style: **structured-checklist**
4. Kiro's agent hooks can trigger skill activation
