use colored::Colorize;

pub fn run() -> Result<(), String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let result = crate::detect::detect_platform(&cwd);

    println!("{}", "CodeHands Platform Detection".bold());
    println!("{}", "============================".dimmed());
    println!();
    println!("  Platform:    {}", result.platform.cyan().bold());
    println!("  Enforcement: {}", result.enforcement_style.yellow());
    println!("  Adapter:     {}", result.adapter_file);
    println!();

    if result.platform == "unknown" {
        println!("{}", "⚠ No platform detected. Copy an adapter file to your project root.".yellow());
        println!("  Run: codehands init --platform <name>");
    } else {
        println!("{}", format!("✅ {} detected", result.platform).green());
    }

    Ok(())
}
