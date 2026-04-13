#!/bin/bash
# CodeHands — Frontmatter Validator
# Validates YAML frontmatter schema for all SKILL.md files.
#
# Usage: bash tests/frontmatter.test.sh [skills_dir]

set -euo pipefail

SKILLS_DIR="${1:-skills}"
PASS=0
FAIL=0

echo "CodeHands Frontmatter Validator"
echo "==============================="
echo ""

VALID_PERMISSIONS=("fs.read" "fs.write" "shell.exec" "net.http" "browser.inspect" "secrets.read")
VALID_PLATFORMS=("claude-code" "cursor" "gemini-cli" "copilot" "codex" "opencode" "windsurf" "kiro")

for skill_dir in "$SKILLS_DIR"/*/; do
    skill_name=$(basename "$skill_dir")
    [ "$skill_name" = "_template" ] && continue

    skill_file="$skill_dir/SKILL.md"
    [ ! -f "$skill_file" ] && continue

    errors=""

    # Extract frontmatter (between first --- and second ---)
    frontmatter=$(awk '/^---$/{n++; next} n==1{print} n==2{exit}' "$skill_file")

    # Check required fields exist
    if ! echo "$frontmatter" | grep -q "^name:"; then
        errors="$errors\n  - Missing: name"
    fi
    if ! echo "$frontmatter" | grep -q "^description:"; then
        errors="$errors\n  - Missing: description"
    fi
    if ! echo "$frontmatter" | grep -q "^version:"; then
        errors="$errors\n  - Missing: version"
    fi

    # Check version follows semver pattern
    version=$(echo "$frontmatter" | grep "^version:" | awk '{print $2}')
    if [ -n "$version" ] && ! echo "$version" | grep -qE "^[0-9]+\.[0-9]+\.[0-9]+$"; then
        errors="$errors\n  - Invalid version format: '$version' (expected semver X.Y.Z)"
    fi

    # Check permissions are valid (if present)
    while IFS= read -r perm; do
        perm=$(echo "$perm" | sed 's/^[[:space:]]*- //' | sed 's/#.*//' | xargs)
        if [ -n "$perm" ]; then
            valid=false
            for vp in "${VALID_PERMISSIONS[@]}"; do
                if [ "$perm" = "$vp" ]; then
                    valid=true
                    break
                fi
            done
            if [ "$valid" = false ]; then
                errors="$errors\n  - Invalid permission: '$perm'"
            fi
        fi
    done < <(echo "$frontmatter" | awk '/^permissions:/,/^[a-z]/' | grep "^  -")

    # Check author field
    if ! echo "$frontmatter" | grep -q "^author:"; then
        errors="$errors\n  - Missing: author"
    fi

    if [ -z "$errors" ]; then
        echo "PASS: $skill_name"
        PASS=$((PASS + 1))
    else
        echo "FAIL: $skill_name$errors"
        FAIL=$((FAIL + 1))
    fi
done

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -gt 0 ]; then
    exit 1
else
    echo "✅ All frontmatter schemas valid"
    exit 0
fi
