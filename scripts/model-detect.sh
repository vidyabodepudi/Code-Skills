#!/bin/bash
# CodeHands — Deterministic Model Detection Hook
# Determines the active LLM by inspecting platform config files.
# NEVER asks the model "what are you?" — that is unreliable.
#
# Usage: source scripts/model-detect.sh && detect_model
# Returns: claude | gemini | copilot | codex | opencode | windsurf | kiro | generic
#
# Detection hierarchy:
#   1. CODEHANDS_MODEL environment variable (user override)
#   2. .codehands/config.yml model field (project config)
#   3. Platform instruction file presence (automatic)
#   4. Fallback: "generic" (no model-specific tuning applied)

set -euo pipefail

detect_model() {
    # Signal 1: Environment variable override (highest priority)
    if [ -n "${CODEHANDS_MODEL:-}" ]; then
        echo "$CODEHANDS_MODEL"
        return 0
    fi

    # Signal 2: Config file override
    if [ -f ".codehands/config.yml" ]; then
        local model
        model=$(grep -E "^model:" .codehands/config.yml 2>/dev/null | awk '{print $2}' | tr -d '"' | tr -d "'")
        if [ -n "$model" ]; then
            echo "$model"
            return 0
        fi
    fi

    # Signal 3: Platform instruction file presence (automatic detection)
    # Check in order of specificity

    # Claude Code
    if [ -f "CLAUDE.md" ] || [ -f ".claude/settings.json" ] || [ -d ".claude" ]; then
        echo "claude"
        return 0
    fi

    # Gemini CLI
    if [ -f "GEMINI.md" ] || [ -f ".gemini/settings.json" ]; then
        echo "gemini"
        return 0
    fi

    # GitHub Copilot
    if [ -f ".github/copilot-instructions.md" ]; then
        echo "copilot"
        return 0
    fi

    # Windsurf
    if [ -f ".windsurfrules" ]; then
        echo "windsurf"
        return 0
    fi

    # Kiro
    if [ -d ".kiro" ] || [ -f ".kiro/manifest.json" ]; then
        echo "kiro"
        return 0
    fi

    # Codex (check env var)
    if [ -n "${CODEX_MODEL:-}" ] || [ -n "${OPENAI_API_KEY:-}" ]; then
        echo "codex"
        return 0
    fi

    # AGENTS.md present (could be Codex or OpenCode)
    if [ -f "AGENTS.md" ]; then
        echo "opencode"
        return 0
    fi

    # Cursor (check for cursor config)
    if [ -d ".cursor" ] || [ -f ".cursor-plugin/manifest.json" ]; then
        echo "cursor"
        return 0
    fi

    # Fallback: generic (no model-specific tuning)
    echo "generic"
    return 0
}

# Get the enforcement style for a detected model
get_enforcement_style() {
    local model="${1:-$(detect_model)}"
    case "$model" in
        claude|cursor)
            echo "human-partner collaborative"
            ;;
        gemini|kiro)
            echo "structured-checklist"
            ;;
        gpt|codex|copilot|opencode)
            echo "directive-imperative"
            ;;
        *)
            echo "structured-checklist"  # Safe default
            ;;
    esac
}

# If run directly (not sourced), print detection results
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    MODEL=$(detect_model)
    STYLE=$(get_enforcement_style "$MODEL")
    echo "CodeHands Model Detection"
    echo "========================="
    echo "Detected model:     $MODEL"
    echo "Enforcement style:  $STYLE"
    echo ""
    echo "Override with: export CODEHANDS_MODEL=<model>"
    echo "Or set in:     .codehands/config.yml → model: <model>"
fi
