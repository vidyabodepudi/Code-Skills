use std::path::Path;
use colored::Colorize;
use walkdir::WalkDir;

use crate::skill;

pub fn run(phase: Option<String>, tier: Option<u8>) -> Result<(), String> {
    let dir = Path::new("skills");
    if !dir.exists() {
        return Err("Skills directory not found. Run from the CodeHands root.".into());
    }

    println!("{}", "CodeHands Skills".bold());
    println!("{}", "================".dimmed());
    println!();

    let mut skills: Vec<(String, String, u8, usize)> = Vec::new();

    for entry in WalkDir::new(dir).max_depth(1).min_depth(1) {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().is_dir() { continue; }
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "_template" { continue; }

        let skill_file = entry.path().join("SKILL.md");
        if !skill_file.exists() { continue; }

        let content = std::fs::read_to_string(&skill_file).unwrap_or_default();
        let tokens = content.len() / 4;
        let phase_name = skill::skill_phase(&name).to_string();

        let skill_tier = match skill::parse_frontmatter(&content) {
            Ok(meta) => meta.tier.unwrap_or(1),
            Err(_) => 1,
        };

        skills.push((name, phase_name, skill_tier, tokens));
    }

    skills.sort_by(|a, b| a.2.cmp(&b.2).then(a.1.cmp(&b.1)).then(a.0.cmp(&b.0)));

    let mut current_tier: Option<u8> = None;

    for (name, phase_name, skill_tier, tokens) in &skills {
        // Apply filters
        if let Some(ref p) = phase {
            if phase_name != p { continue; }
        }
        if let Some(t) = tier {
            if *skill_tier != t { continue; }
        }

        // Print tier header
        if current_tier != Some(*skill_tier) {
            if current_tier.is_some() { println!(); }
            let tier_label = match skill_tier {
                0 => "Tier 0 — Always On",
                1 => "Tier 1 — Phase-Activated",
                2 => "Tier 2 — Specialist",
                _ => "Unknown Tier",
            };
            println!("  {}", tier_label.cyan().bold());
            current_tier = Some(*skill_tier);
        }

        let tier_badge = match skill_tier {
            0 => "T0".green(),
            1 => "T1".yellow(),
            2 => "T2".dimmed(),
            _ => "T?".dimmed(),
        };

        println!("    {} {:8} {:40} ~{} tokens",
            tier_badge, phase_name.dimmed(), name, tokens);
    }

    println!();
    println!("  Total: {} skills", skills.len().to_string().bold());

    Ok(())
}
