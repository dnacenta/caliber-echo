//! pulse-echo — Operational self-model and capability mapping
//!
//! Manages PULSE.md and outcome tracking for AI entities.
//! Records what was attempted, what happened, and how predictions
//! compared to reality. Foundation for trajectory mining and
//! external calibration.

pub mod outcome;
pub mod paths;
pub mod runtime;
pub mod state;

use std::path::{Path, PathBuf};

use echo_system_types::{HealthStatus, SetupPrompt};

/// Main pulse-echo struct. Holds the path to the entity's documents.
pub struct PulseEcho {
    docs_dir: PathBuf,
}

impl PulseEcho {
    pub fn new(docs_dir: PathBuf) -> Self {
        Self { docs_dir }
    }

    /// Resolve from environment or home directory.
    pub fn from_default() -> Result<Self, String> {
        let docs_dir = paths::docs_dir()?;
        Ok(Self { docs_dir })
    }

    pub fn docs_dir(&self) -> &Path {
        &self.docs_dir
    }

    /// Check health by verifying PULSE.md exists and outcome state is readable.
    pub fn health(&self) -> HealthStatus {
        let pulse_path = self.docs_dir.join("PULSE.md");
        if !pulse_path.exists() {
            return HealthStatus::Down("PULSE.md not found".to_string());
        }

        let pulse_dir = self.docs_dir.join("pulse");
        if !pulse_dir.exists() {
            return HealthStatus::Degraded(
                "pulse/ directory missing — no outcome tracking yet".to_string(),
            );
        }

        HealthStatus::Healthy
    }

    /// Setup prompts for the init wizard.
    pub fn setup_prompts() -> Vec<SetupPrompt> {
        vec![SetupPrompt {
            key: "docs_dir".into(),
            question: "Entity documents directory (where PULSE.md lives):".into(),
            required: true,
            secret: false,
            default: Some("./".into()),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn health_down_when_no_pulse_md() {
        let dir = TempDir::new().unwrap();
        let pulse = PulseEcho::new(dir.path().to_path_buf());
        assert!(matches!(pulse.health(), HealthStatus::Down(_)));
    }

    #[test]
    fn health_degraded_when_no_pulse_dir() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("PULSE.md"), "# Pulse").unwrap();
        let pulse = PulseEcho::new(dir.path().to_path_buf());
        assert!(matches!(pulse.health(), HealthStatus::Degraded(_)));
    }

    #[test]
    fn health_healthy_when_everything_exists() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("PULSE.md"), "# Pulse").unwrap();
        std::fs::create_dir(dir.path().join("pulse")).unwrap();
        let pulse = PulseEcho::new(dir.path().to_path_buf());
        assert!(matches!(pulse.health(), HealthStatus::Healthy));
    }

    #[test]
    fn setup_prompts_not_empty() {
        let prompts = PulseEcho::setup_prompts();
        assert!(!prompts.is_empty());
        assert_eq!(prompts[0].key, "docs_dir");
    }
}
