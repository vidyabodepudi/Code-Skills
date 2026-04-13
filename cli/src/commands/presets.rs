use std::path::Path;
use colored::Colorize;

pub fn run() -> Result<(), String> {
    let presets_path = Path::new(".codehands/presets.yml");
    if !presets_path.exists() {
        return Err("No presets.yml found. Run from a CodeHands project root.".into());
    }

    let content = std::fs::read_to_string(presets_path).map_err(|e| e.to_string())?;

    println!("{}", "CodeHands Presets".bold());
    println!("{}", "=================".dimmed());
    println!();

    // Parse preset names and descriptions from YAML
    let doc: serde_yaml::Value = serde_yaml::from_str(&content).map_err(|e| e.to_string())?;

    if let Some(presets) = doc.get("presets").and_then(|v| v.as_mapping()) {
        for (key, value) in presets {
            let name = key.as_str().unwrap_or("unknown");
            let desc = value.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("No description");
            let chain = value.get("skill_chain")
                .and_then(|v| v.as_sequence())
                .map(|s| s.len())
                .unwrap_or(0);
            let overhead = value.get("estimated_overhead")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            println!("  {} {}", "▸".cyan(), name.bold());
            println!("    {}", desc.dimmed());
            println!("    Skills: {} | Overhead: {}", chain.to_string().yellow(), overhead);
            println!();
        }
    }

    Ok(())
}
