use clap::{Parser, Subcommand};

mod commands;
mod skill;
mod config;
mod detect;

#[derive(Parser)]
#[command(name = "codehands")]
#[command(about = "CLI companion for the CodeHands agent skills framework")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize CodeHands in a project
    Init {
        /// Target platform adapter
        #[arg(long, default_value = "claude")]
        platform: String,

        /// Project directory (default: current)
        #[arg(default_value = ".")]
        path: String,
    },

    /// Validate all skills (format, frontmatter, cross-references)
    Validate {
        /// Skills directory path
        #[arg(long, default_value = "skills")]
        skills_dir: String,
    },

    /// Detect the current platform and model
    Detect,

    /// List skills, optionally filtered by phase or tier
    Skills {
        /// Filter by phase: define, plan, build, orchestrate, verify, review, ship, govern
        #[arg(long)]
        phase: Option<String>,

        /// Filter by tier: 0, 1, 2
        #[arg(long)]
        tier: Option<u8>,
    },

    /// Show available presets
    Presets,

    /// Generate an audit report from project state
    Audit {
        /// Output format: markdown, json
        #[arg(long, default_value = "markdown")]
        format: String,
    },

    /// Show project info and CodeHands status
    Status,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { platform, path } => commands::init::run(&platform, &path),
        Commands::Validate { skills_dir } => commands::validate::run(&skills_dir),
        Commands::Detect => commands::detect::run(),
        Commands::Skills { phase, tier } => commands::skills::run(phase, tier),
        Commands::Presets => commands::presets::run(),
        Commands::Audit { format } => commands::audit::run(&format),
        Commands::Status => commands::status::run(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
