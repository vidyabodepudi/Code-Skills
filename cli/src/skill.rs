use std::path::Path;
use std::fs;
use serde::Deserialize;
use regex::Regex;

/// Represents a parsed skill from SKILL.md frontmatter
#[derive(Debug, Deserialize, Clone)]
pub struct SkillMeta {
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub tier: Option<u8>,
    pub author: Option<String>,
    #[serde(default)]
    pub signed: bool,
}

/// Parse YAML frontmatter from a SKILL.md file
pub fn parse_frontmatter(content: &str) -> Result<SkillMeta, String> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("No YAML frontmatter found (missing --- delimiters)".into());
    }
    let yaml_str = parts[1].trim();
    serde_yaml::from_str(yaml_str).map_err(|e| format!("YAML parse error: {}", e))
}

/// Required sections in every SKILL.md
pub const REQUIRED_SECTIONS: &[&str] = &[
    "## Overview",
    "## When to Use",
    "## Process",
    "## Common Rationalizations",
    "## Red Flags",
    "## Verification",
    "## See Also",
];

/// Valid permissions
pub const VALID_PERMISSIONS: &[&str] = &[
    "fs.read", "fs.write", "shell.exec", "net.http", "browser.inspect", "secrets.read",
];

/// Check a single skill for format compliance
pub fn validate_skill(skill_dir: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    let skill_file = skill_dir.join("SKILL.md");

    if !skill_file.exists() {
        errors.push("No SKILL.md found".into());
        return errors;
    }

    let content = match fs::read_to_string(&skill_file) {
        Ok(c) => c,
        Err(e) => {
            errors.push(format!("Cannot read file: {}", e));
            return errors;
        }
    };

    // Check frontmatter
    match parse_frontmatter(&content) {
        Ok(meta) => {
            // Check name matches directory
            let dir_name = skill_dir.file_name().unwrap().to_str().unwrap();
            if meta.name != dir_name {
                errors.push(format!(
                    "Name mismatch: frontmatter='{}' directory='{}'",
                    meta.name, dir_name
                ));
            }

            // Check version is semver
            let semver_re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
            if !semver_re.is_match(&meta.version) {
                errors.push(format!("Invalid version: '{}' (expected X.Y.Z)", meta.version));
            }

            // Check permissions are valid
            for perm in &meta.permissions {
                if !VALID_PERMISSIONS.contains(&perm.as_str()) {
                    errors.push(format!("Invalid permission: '{}'", perm));
                }
            }
        }
        Err(e) => errors.push(e),
    }

    // Check required sections
    for section in REQUIRED_SECTIONS {
        if !content.contains(section) {
            errors.push(format!("Missing section: {}", section));
        }
    }

    // Token estimate
    let token_estimate = content.len() / 4;
    if token_estimate > 2500 {
        errors.push(format!(
            "Estimated ~{} tokens (budget: 2000, warning threshold: 2500)",
            token_estimate
        ));
    }

    errors
}

/// Extract all codehands: references from content
pub fn extract_refs(content: &str) -> Vec<String> {
    let re = Regex::new(r"codehands:([a-z][a-z0-9-]+)").unwrap();
    re.captures_iter(content)
        .map(|c| c[1].to_string())
        .filter(|s| s != "skill-name")
        .collect()
}

/// Determine the phase for a skill based on its directory position in the adapter
pub fn skill_phase(name: &str) -> &'static str {
    match name {
        "task-decomposition-engine" | "codebase-reconnaissance" | "risk-assessment-and-escalation" => "engine",
        "brainstorming" | "spec-driven-development" => "define",
        "planning-and-task-breakdown" | "context-engineering" => "plan",
        "test-driven-development" | "incremental-implementation" | "source-driven-development"
        | "frontend-ui-engineering" | "api-and-interface-design" | "using-git-worktrees" => "build",
        "multi-agent-orchestration" | "executing-plans" => "orchestrate",
        "systematic-debugging" | "browser-testing-with-devtools" | "verification-before-completion" => "verify",
        "code-review-and-quality" | "code-simplification" | "security-and-hardening" | "performance-optimization" => "review",
        "finishing-a-development-branch" | "git-workflow-and-versioning" | "ci-cd-and-shipping"
        | "deprecation-and-migration" | "documentation-and-adrs" => "ship",
        "agent-security" | "audit-and-governance" => "govern",
        "writing-skills" | "using-codehands" => "meta",
        _ => "unknown",
    }
}
