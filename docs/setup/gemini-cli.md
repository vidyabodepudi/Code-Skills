# Gemini CLI — CodeHands Setup Guide

## Quick Start

```bash
cp adapters/GEMINI.md your-project/GEMINI.md
cp -r skills/ your-project/.codehands/skills/
cp -r agents/ your-project/.codehands/agents/
cp -r references/ your-project/.codehands/references/
cp -r commands/ your-project/.codehands/commands/
```

## How It Works

1. Gemini CLI reads `GEMINI.md` from your project root at session start
2. The adapter loads CodeHands with **structured-checklist** enforcement style
3. Phase gates use numbered checklists with explicit ☐ markers
4. Skills auto-activate based on task context

## Enforcement Style

Gemini receives **structured-checklist** enforcement:
- Numbered checklists instead of prose instructions
- Explicit phase gates: `☐ Tests pass ☐ Build succeeds ☐ Requirements met`
- Clear PASS/FAIL markers at each gate

## Verification

```
You: Run the /test command
Expected: Gemini follows the verification checklist from commands/test.md
```
