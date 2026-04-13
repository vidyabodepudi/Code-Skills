use std::fs;
use std::path::Path;
use colored::Colorize;

const ADAPTERS: &[(&str, &str, &str)] = &[
    ("claude", "CLAUDE.md", "adapters/CLAUDE.md"),
    ("gemini", "GEMINI.md", "adapters/GEMINI.md"),
    ("cursor", ".cursor/rules/codehands.md", "adapters/CLAUDE.md"),
    ("copilot", ".github/copilot-instructions.md", "adapters/copilot-instructions.md"),
    ("codex", "AGENTS.md", "adapters/AGENTS.md"),
    ("opencode", "AGENTS.md", "adapters/AGENTS.md"),
    ("windsurf", ".windsurfrules", "adapters/.windsurfrules"),
    ("kiro", ".kiro/rules/codehands.md", "adapters/CLAUDE.md"),
];

pub fn run(platform: &str, path: &str) -> Result<(), String> {
    let project_root = Path::new(path);

    // Find the CodeHands installation (try relative paths)
    let codehands_root = find_codehands_root()?;

    let adapter = ADAPTERS.iter()
        .find(|(p, _, _)| *p == platform)
        .ok_or_else(|| format!("Unknown platform: '{}'. Valid: claude, gemini, cursor, copilot, codex, opencode, windsurf, kiro", platform))?;

    let source = codehands_root.join(adapter.2);
    let dest = project_root.join(adapter.1);

    if !source.exists() {
        return Err(format!("Adapter source not found: {}", source.display()));
    }

    // Create parent directories
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Cannot create directory: {}", e))?;
    }

    // Copy adapter
    fs::copy(&source, &dest).map_err(|e| format!("Cannot copy adapter: {}", e))?;

    // Copy skills directory
    let skills_src = codehands_root.join("skills");
    let skills_dst = project_root.join("skills");
    if skills_src.exists() && !skills_dst.exists() {
        copy_dir_recursive(&skills_src, &skills_dst)?;
    }

    // Copy .codehands config
    let config_src = codehands_root.join(".codehands");
    let config_dst = project_root.join(".codehands");
    if config_src.exists() && !config_dst.exists() {
        copy_dir_recursive(&config_src, &config_dst)?;
    }

    println!("{}", "✅ CodeHands initialized!".green().bold());
    println!("   Platform: {}", platform.cyan());
    println!("   Adapter:  {}", adapter.1);
    println!("   Skills:   skills/");
    println!("   Config:   .codehands/config.yml");
    println!();
    println!("Next steps:");
    println!("  1. Review .codehands/config.yml for project settings");
    println!("  2. Run {} to verify skills", "codehands validate".yellow());
    println!("  3. Start coding with CodeHands active");

    Ok(())
}

fn find_codehands_root() -> Result<std::path::PathBuf, String> {
    // Check environment variable first
    if let Ok(home) = std::env::var("CODEHANDS_HOME") {
        let p = Path::new(&home).to_path_buf();
        if p.join("skills").exists() {
            return Ok(p);
        }
    }

    // Check current directory and parent
    let cwd = std::env::current_dir().unwrap_or_default();
    let candidates: Vec<std::path::PathBuf> = vec![
        cwd.clone(),
        cwd.join(".."),
    ];

    for candidate in &candidates {
        if candidate.join("skills").exists() && candidate.join("adapters").exists() {
            return Ok(candidate.to_path_buf());
        }
    }

    Err("Cannot find CodeHands installation. Run from the CodeHands directory or set CODEHANDS_HOME.".into())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| format!("Cannot create {}: {}", dst.display(), e))?;
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let dst_path = dst.join(entry.file_name());
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            copy_dir_recursive(&entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), dst_path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
