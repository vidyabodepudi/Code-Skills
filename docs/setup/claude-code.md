# Claude Code — CodeHands Setup Guide

## Quick Start (30 seconds)

```bash
# From the CodeHands repository
cp adapters/CLAUDE.md your-project/CLAUDE.md
cp -r skills/ your-project/.codehands/skills/
cp -r agents/ your-project/.codehands/agents/
cp -r references/ your-project/.codehands/references/
cp -r commands/ your-project/.codehands/commands/
```

## How It Works

1. Claude Code automatically reads `CLAUDE.md` from your project root at session start
2. The adapter loads CodeHands principles, hard rules, and the skill directory
3. Skills auto-activate based on context (what you're doing triggers the right skill)
4. Slash commands (`/spec`, `/plan`, `/build`, etc.) activate skill groups
5. Model detection identifies Claude → uses "human-partner collaborative" enforcement style

## What's Included

- **33 skills** covering the full SDLC lifecycle
- **5 agent personas** for specialized reviews
- **7 reference checklists** (security, testing, performance, accessibility, OWASP)
- **8 slash commands** for phase activation
- **Anti-rationalization tables** that prevent agents from cutting corners

## Platform-Specific Features

| Feature | Support |
|---|---|
| Skill auto-loading | ✅ |
| Slash commands | ✅ |
| Subagent dispatch | ✅ (native) |
| Browser DevTools | ✅ (native MCP) |
| Git worktrees | ✅ |
| Model variants | ✅ (claude enforcement style) |

## Verification

After setup, start a Claude Code session and verify:
```
You: What CodeHands skills are you aware of?
Expected: Claude lists the skill directory from CLAUDE.md
```

## Customization

Edit `CLAUDE.md` to:
- Add project-specific rules
- Disable specific skills
- Add custom slash commands
- Override enforcement style
