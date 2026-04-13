use std::path::Path;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub tiers: TierConfig,
    #[serde(default)]
    pub default_preset: Option<String>,
    #[serde(default)]
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct TierConfig {
    #[serde(default)]
    pub always_on: Vec<String>,
    #[serde(default)]
    pub never_load: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct SecurityConfig {
    #[serde(default = "default_review_threshold")]
    pub review_threshold: String,
    #[serde(default)]
    pub require_human_approval: bool,
    #[serde(default)]
    pub high_risk_patterns: Vec<String>,
}

fn default_review_threshold() -> String {
    "medium".to_string()
}

/// Load config from .codehands/config.yml
pub fn load_config(project_root: &Path) -> Config {
    let config_path = project_root.join(".codehands/config.yml");
    if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(content) => serde_yaml::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    } else {
        Config::default()
    }
}
