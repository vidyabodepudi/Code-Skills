use colored::Colorize;
use std::path::Path;

pub fn run(format: &str) -> Result<(), String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;

    // Count skills
    let skills_dir = Path::new("skills");
    let skill_count = if skills_dir.exists() {
        std::fs::read_dir(skills_dir)
            .map(|entries| entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir() && e.file_name() != "_template")
                .count())
            .unwrap_or(0)
    } else { 0 };

    // Check for test infrastructure
    let has_tests = Path::new("tests").exists();
    let has_ci = Path::new(".github/workflows").exists()
        || Path::new(".gitlab-ci.yml").exists();

    // Check security
    let has_audit_dir = Path::new(".codehands/audit").exists();

    // Platform detection
    let platform = crate::detect::detect_platform(&cwd);

    match format {
        "json" => {
            println!("{{");
            println!("  \"platform\": \"{}\",", platform.platform);
            println!("  \"skills\": {},", skill_count);
            println!("  \"has_tests\": {},", has_tests);
            println!("  \"has_ci\": {},", has_ci);
            println!("  \"audit_dir\": {}", has_audit_dir);
            println!("}}");
        }
        _ => {
            println!("{}", "CodeHands Audit Report".bold());
            println!("{}", "======================".dimmed());
            println!();
            println!("  Platform:     {}", platform.platform.cyan());
            println!("  Skills:       {}", skill_count);
            println!("  Test infra:   {}", if has_tests { "✅ Present".green() } else { "❌ Missing".red() });
            println!("  CI pipeline:  {}", if has_ci { "✅ Present".green() } else { "❌ Missing".red() });
            println!("  Audit trail:  {}", if has_audit_dir { "✅ Active".green() } else { "⚠ Not initialized".yellow() });
            println!();

            if !has_audit_dir {
                println!("  {}", "Run `mkdir -p .codehands/audit` to enable audit logging.".dimmed());
            }
        }
    }

    Ok(())
}
