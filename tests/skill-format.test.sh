#!/bin/bash
# CodeHands — Skill Format Validator
# Validates that all SKILL.md files conform to the CodeHands specification.
#
# Usage: bash tests/skill-format.test.sh [skills_dir]
# Default: validates all skills in skills/

set -euo pipefail

SKILLS_DIR="${1:-skills}"
PASS=0
FAIL=0
WARNINGS=0

echo "CodeHands Skill Format Validator"
echo "================================"
echo ""

# Required sections
REQUIRED_SECTIONS=("## Overview" "## When to Use" "## Process" "## Common Rationalizations" "## Red Flags" "## Verification" "## See Also")

for skill_dir in "$SKILLS_DIR"/*/; do
    skill_name=$(basename "$skill_dir")

    # Skip template
    if [ "$skill_name" = "_template" ]; then
        continue
    fi

    skill_file="$skill_dir/SKILL.md"

    if [ ! -f "$skill_file" ]; then
        echo "FAIL: $skill_name — No SKILL.md found"
        FAIL=$((FAIL + 1))
        continue
    fi

    errors=""

    # Check frontmatter exists
    if ! head -1 "$skill_file" | grep -q "^---$"; then
        errors="$errors\n  - Missing YAML frontmatter"
    fi

    # Check required frontmatter fields
    for field in "name:" "description:" "version:"; do
        if ! grep -q "^$field" "$skill_file"; then
            errors="$errors\n  - Missing frontmatter field: $field"
        fi
    done

    # Check required sections
    for section in "${REQUIRED_SECTIONS[@]}"; do
        if ! grep -q "^$section" "$skill_file"; then
            errors="$errors\n  - Missing section: $section"
        fi
    done

    # Check name matches directory
    yaml_name=$(grep "^name:" "$skill_file" | head -1 | awk '{print $2}' | tr -d '"' | tr -d "'")
    if [ -n "$yaml_name" ] && [ "$yaml_name" != "$skill_name" ]; then
        errors="$errors\n  - Name mismatch: frontmatter='$yaml_name' directory='$skill_name'"
    fi

    # Check description length (approximate — count characters in description block)
    desc_line=$(grep "^description:" "$skill_file" | head -1)
    if [ ${#desc_line} -gt 1100 ]; then
        errors="$errors\n  - Description may exceed 1024 character limit"
    fi

    # Token count estimate (rough: 1 token ≈ 4 chars)
    char_count=$(wc -c < "$skill_file")
    token_estimate=$((char_count / 4))
    if [ "$token_estimate" -gt 2500 ]; then
        echo "WARN: $skill_name — Estimated ~$token_estimate tokens (budget: 2000)"
        WARNINGS=$((WARNINGS + 1))
    fi

    if [ -z "$errors" ]; then
        echo "PASS: $skill_name (~$token_estimate tokens)"
        PASS=$((PASS + 1))
    else
        echo "FAIL: $skill_name$errors"
        FAIL=$((FAIL + 1))
    fi
done

echo ""
echo "Results: $PASS passed, $FAIL failed, $WARNINGS warnings"
echo ""

if [ "$FAIL" -gt 0 ]; then
    echo "❌ Validation failed"
    exit 1
else
    echo "✅ All skills pass format validation"
    exit 0
fi
