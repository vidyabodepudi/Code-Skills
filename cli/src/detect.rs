use std::path::Path;

/// Detected platform result
pub struct DetectResult {
    pub platform: String,
    pub enforcement_style: String,
    pub adapter_file: String,
}

/// Deterministic 4-signal model detection
pub fn detect_platform(project_root: &Path) -> DetectResult {
    // Signal 1: Environment variable
    if std::env::var("CODEX_MODEL").is_ok() {
        return DetectResult {
            platform: "codex".into(),
            enforcement_style: "directive-imperative".into(),
            adapter_file: "AGENTS.md".into(),
        };
    }

    // Signal 2: Platform-specific files
    if project_root.join("CLAUDE.md").exists() || project_root.join(".claude").exists() {
        return DetectResult {
            platform: "claude-code".into(),
            enforcement_style: "human-partner collaborative".into(),
            adapter_file: "CLAUDE.md".into(),
        };
    }

    if project_root.join("GEMINI.md").exists() || project_root.join(".gemini").exists() {
        return DetectResult {
            platform: "gemini-cli".into(),
            enforcement_style: "structured-checklist".into(),
            adapter_file: "GEMINI.md".into(),
        };
    }

    if project_root.join(".cursor").exists() {
        return DetectResult {
            platform: "cursor".into(),
            enforcement_style: "human-partner collaborative".into(),
            adapter_file: ".cursor-plugin/manifest.json".into(),
        };
    }

    if project_root.join(".github/copilot-instructions.md").exists() {
        return DetectResult {
            platform: "copilot".into(),
            enforcement_style: "directive-imperative".into(),
            adapter_file: "copilot-instructions.md".into(),
        };
    }

    if project_root.join(".windsurfrules").exists() {
        return DetectResult {
            platform: "windsurf".into(),
            enforcement_style: "directive-imperative".into(),
            adapter_file: ".windsurfrules".into(),
        };
    }

    if project_root.join(".kiro").exists() {
        return DetectResult {
            platform: "kiro".into(),
            enforcement_style: "structured-checklist".into(),
            adapter_file: "kiro/manifest.json".into(),
        };
    }

    if project_root.join("AGENTS.md").exists() {
        return DetectResult {
            platform: "opencode".into(),
            enforcement_style: "directive-imperative".into(),
            adapter_file: "AGENTS.md".into(),
        };
    }

    // Signal 3: Fallback
    DetectResult {
        platform: "unknown".into(),
        enforcement_style: "structured-checklist".into(),
        adapter_file: "none".into(),
    }
}
