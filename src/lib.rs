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

use std::any::Any;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use echo_system_types::plugin::{Plugin, PluginContext, PluginResult, PluginRole};
use echo_system_types::{HealthStatus, PluginMeta, SetupPrompt};

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
    fn health_check(&self) -> HealthStatus {
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
    fn get_setup_prompts() -> Vec<SetupPrompt> {
        vec![SetupPrompt {
            key: "docs_dir".into(),
            question: "Entity documents directory (where PULSE.md lives):".into(),
            required: true,
            secret: false,
            default: Some("./".into()),
        }]
    }
}

/// Factory function — creates a fully initialized plugin.
pub async fn create(
    config: &serde_json::Value,
    ctx: &PluginContext,
) -> Result<Box<dyn Plugin>, Box<dyn std::error::Error + Send + Sync>> {
    let docs_dir = config
        .get("docs_dir")
        .and_then(|v| v.as_str())
        .map(PathBuf::from)
        .unwrap_or_else(|| ctx.entity_root.clone());

    Ok(Box::new(PulseEcho::new(docs_dir)))
}

impl Plugin for PulseEcho {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "pulse-echo".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            description: "Operational self-model and outcome tracking".into(),
        }
    }

    fn role(&self) -> PluginRole {
        PluginRole::Outcome
    }

    fn start(&mut self) -> PluginResult<'_> {
        Box::pin(async { Ok(()) })
    }

    fn stop(&mut self) -> PluginResult<'_> {
        Box::pin(async { Ok(()) })
    }

    fn health(&self) -> Pin<Box<dyn Future<Output = HealthStatus> + Send + '_>> {
        Box::pin(async move { self.health_check() })
    }

    fn setup_prompts(&self) -> Vec<SetupPrompt> {
        Self::get_setup_prompts()
    }

    fn as_any(&self) -> &dyn Any {
        self
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
        assert!(matches!(pulse.health_check(), HealthStatus::Down(_)));
    }

    #[test]
    fn health_degraded_when_no_pulse_dir() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("PULSE.md"), "# Pulse").unwrap();
        let pulse = PulseEcho::new(dir.path().to_path_buf());
        assert!(matches!(pulse.health_check(), HealthStatus::Degraded(_)));
    }

    #[test]
    fn health_healthy_when_everything_exists() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("PULSE.md"), "# Pulse").unwrap();
        std::fs::create_dir(dir.path().join("pulse")).unwrap();
        let pulse = PulseEcho::new(dir.path().to_path_buf());
        assert!(matches!(pulse.health_check(), HealthStatus::Healthy));
    }

    #[test]
    fn setup_prompts_not_empty() {
        let prompts = PulseEcho::get_setup_prompts();
        assert!(!prompts.is_empty());
        assert_eq!(prompts[0].key, "docs_dir");
    }

    #[test]
    fn plugin_meta() {
        let pulse = PulseEcho::new(PathBuf::from("/tmp"));
        let meta = pulse.meta();
        assert_eq!(meta.name, "pulse-echo");
    }

    #[test]
    fn plugin_role_is_outcome() {
        let pulse = PulseEcho::new(PathBuf::from("/tmp"));
        assert_eq!(pulse.role(), PluginRole::Outcome);
    }
}
