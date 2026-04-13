# Windsurf — CodeHands Setup Guide

## Quick Start

```bash
cp adapters/.windsurfrules your-project/.windsurfrules
cp -r skills/ your-project/.codehands/skills/
cp -r agents/ your-project/.codehands/agents/
cp -r references/ your-project/.codehands/references/
cp -r commands/ your-project/.codehands/commands/
```

## How It Works

1. Windsurf reads `.windsurfrules` from your project root
2. Model detection: `.windsurfrules` presence → Windsurf detected
3. Enforcement style: **directive-imperative** (MUST/SHALL language)
4. Skills auto-activate based on context
