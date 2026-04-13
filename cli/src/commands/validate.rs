use std::path::Path;
use std::collections::HashSet;
use colored::Colorize;
use walkdir::WalkDir;

use crate::skill;

pub fn run(skills_dir: &str) -> Result<(), String> {
    let dir = Path::new(skills_dir);
    if !dir.exists() {
        return Err(format!("Skills directory not found: {}", skills_dir));
    }

    println!("{}", "CodeHands Skill Validator".bold());
    println!("{}", "========================".dimmed());
    println!();

    let mut pass = 0u32;
    let mut fail = 0u32;
    let mut warn = 0u32;
    let mut all_refs: HashSet<String> = HashSet::new();
    let mut all_names: HashSet<String> = HashSet::new();

    // Collect all skill directories
    let mut skill_dirs: Vec<_> = Vec::new();
    for entry in WalkDir::new(dir).max_depth(1).min_depth(1) {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_type().is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name == "_template" {
                continue;
            }
            skill_dirs.push(entry.into_path());
        }
    }

    skill_dirs.sort();

    // Phase 1: Format + frontmatter validation
    println!("{}", "── Format & Frontmatter ──".cyan());
    for skill_dir in &skill_dirs {
        let name = skill_dir.file_name().unwrap().to_str().unwrap();
        let errors = skill::validate_skill(skill_dir);

        if errors.is_empty() {
            let skill_file = skill_dir.join("SKILL.md");
            let content = std::fs::read_to_string(&skill_file).unwrap_or_default();
            let tokens = content.len() / 4;

            if tokens > 2000 {
                println!("  {} {} (~{} tokens, over budget)", "WARN".yellow(), name, tokens);
                warn += 1;
            } else {
                println!("  {} {} (~{} tokens)", "PASS".green(), name, tokens);
            }
            pass += 1;
            all_names.insert(name.to_string());

            // Collect refs
            for r in skill::extract_refs(&content) {
                all_refs.insert(r);
            }
        } else {
            println!("  {} {}", "FAIL".red().bold(), name);
            for e in &errors {
                println!("       └─ {}", e);
            }
            fail += 1;
        }
    }

    // Phase 2: Cross-reference validation
    println!();
    println!("{}", "── Cross-References ──".cyan());
    let mut broken = 0u32;
    for ref_name in &all_refs {
        if all_names.contains(ref_name.as_str()) {
            println!("  {} codehands:{}", "OK".green(), ref_name);
        } else {
            println!("  {} codehands:{} → no matching skill directory", "BROKEN".red().bold(), ref_name);
            broken += 1;
        }
    }

    // Summary
    println!();
    println!("{}", "── Summary ──".cyan().bold());
    println!("  Skills:      {} passed, {} failed, {} warnings", pass.to_string().green(), fail.to_string().red(), warn.to_string().yellow());
    println!("  References:  {} resolved, {} broken", (all_refs.len() as u32 - broken).to_string().green(), broken.to_string().red());
    println!();

    if fail > 0 || broken > 0 {
        println!("{}", "❌ Validation failed".red().bold());
        Err("Validation failed".into())
    } else {
        println!("{}", "✅ All validations passed".green().bold());
        Ok(())
    }
}
