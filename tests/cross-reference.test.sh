#!/bin/bash
# CodeHands — Cross-Reference Validator
# Validates that all codehands: references resolve to actual skill directories.
#
# Usage: bash tests/cross-reference.test.sh [project_root]

ROOT="${1:-.}"
SKILLS_DIR="$ROOT/skills"
PASS=0
FAIL=0
TOTAL_REFS=0

echo "CodeHands Cross-Reference Validator"
echo "===================================="
echo ""

# Extract all unique codehands:xxx references
refs=$(grep -roE 'codehands:[a-z][a-z0-9-]+' "$ROOT" --include="*.md" 2>/dev/null | \
       awk -F: '{print $NF}' | \
       sort -u | \
       grep -v "^skill-name$")

for skill_name in $refs; do
    TOTAL_REFS=$((TOTAL_REFS + 1))

    if [ -f "$SKILLS_DIR/$skill_name/SKILL.md" ]; then
        PASS=$((PASS + 1))
    else
        echo "FAIL: codehands:$skill_name — skill not found"
        FAIL=$((FAIL + 1))
    fi
done

echo ""
echo "Unique references: $TOTAL_REFS"
echo "Results: $PASS resolved, $FAIL broken"

if [ "$FAIL" -gt 0 ]; then
    exit 1
else
    echo "✅ All cross-references resolve"
    exit 0
fi
