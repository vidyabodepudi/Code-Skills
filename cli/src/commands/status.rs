use colored::Colorize;
use std::path::Path;

pub fn run() -> Result<(), String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let platform = crate::detect::detect_platform(&cwd);

    println!("{}", "CodeHands Status".bold());
    println!("{}", "=================".dimmed());
    println!();

    // Platform
    println!("  {} {}", "Platform:".dimmed(), platform.platform.cyan().bold());
    println!("  {} {}", "Style:".dimmed(), platform.enforcement_style);
    println!();

    // Skills count
    let skills_dir = Path::new("skills");
    if skills_dir.exists() {
        let count: usize = std::fs::read_dir(skills_dir)
            .map(|entries| entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir() && e.file_name() != "_template")
                .count())
            .unwrap_or(0);

        let (t0, t1, t2) = count_tiers(skills_dir);
        println!("  {} {} skills ({} T0, {} T1, {} T2)",
            "Skills:".dimmed(), count.to_string().bold(),
            t0.to_string().green(), t1.to_string().yellow(), t2.to_string().dimmed());
    } else {
        println!("  {} {}", "Skills:".dimmed(), "Not found".red());
    }

    // Config
    let config = Path::new(".codehands/config.yml");
    println!("  {} {}", "Config:".dimmed(),
        if config.exists() { "✅ Present".green() } else { "❌ Missing".red() });

    // Presets
    let presets = Path::new(".codehands/presets.yml");
    println!("  {} {}", "Presets:".dimmed(),
        if presets.exists() { "✅ Present".green() } else { "❌ Missing".red() });

    // Session state
    let session = Path::new(".codehands/session-state.md");
    println!("  {} {}", "Session:".dimmed(),
        if session.exists() { "🔄 Active session found".yellow() } else { "No active session".dimmed() });

    println!();
    Ok(())
}

fn count_tiers(skills_dir: &Path) -> (usize, usize, usize) {
    let mut t0 = 0;
    let mut t1 = 0;
    let mut t2 = 0;

    if let Ok(entries) = std::fs::read_dir(skills_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if !entry.path().is_dir() { continue; }
            let skill_file = entry.path().join("SKILL.md");
            if let Ok(content) = std::fs::read_to_string(&skill_file) {
                if content.contains("tier: 0") { t0 += 1; }
                else if content.contains("tier: 2") { t2 += 1; }
                else { t1 += 1; }
            }
        }
    }

    (t0, t1, t2)
}
