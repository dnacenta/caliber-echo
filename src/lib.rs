//! caliber-echo — Operational self-model and capability mapping
//!
//! Manages CALIBER.md and outcome tracking for AI entities.
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

/// Main caliber-echo struct. Holds the path to the entity's documents.
pub struct CaliberEcho {
    docs_dir: PathBuf,
}

impl CaliberEcho {
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

    /// Check health by verifying CALIBER.md exists and outcome state is readable.
    fn health_check(&self) -> HealthStatus {
        let caliber_path = self.docs_dir.join("CALIBER.md");
        if !caliber_path.exists() {
            return HealthStatus::Down("CALIBER.md not found".to_string());
        }

        let caliber_dir = self.docs_dir.join("caliber");
        if !caliber_dir.exists() {
            return HealthStatus::Degraded(
                "caliber/ directory missing — no outcome tracking yet".to_string(),
            );
        }

        HealthStatus::Healthy
    }

    /// Setup prompts for the init wizard.
    fn get_setup_prompts() -> Vec<SetupPrompt> {
        vec![SetupPrompt {
            key: "docs_dir".into(),
            question: "Entity documents directory (where CALIBER.md lives):".into(),
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

    Ok(Box::new(CaliberEcho::new(docs_dir)))
}

impl Plugin for CaliberEcho {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "caliber-echo".into(),
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
    fn health_down_when_no_caliber_md() {
        let dir = TempDir::new().unwrap();
        let caliber = CaliberEcho::new(dir.path().to_path_buf());
        assert!(matches!(caliber.health_check(), HealthStatus::Down(_)));
    }

    #[test]
    fn health_degraded_when_no_caliber_dir() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("CALIBER.md"), "# Caliber").unwrap();
        let caliber = CaliberEcho::new(dir.path().to_path_buf());
        assert!(matches!(caliber.health_check(), HealthStatus::Degraded(_)));
    }

    #[test]
    fn health_healthy_when_everything_exists() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("CALIBER.md"), "# Caliber").unwrap();
        std::fs::create_dir(dir.path().join("caliber")).unwrap();
        let caliber = CaliberEcho::new(dir.path().to_path_buf());
        assert!(matches!(caliber.health_check(), HealthStatus::Healthy));
    }

    #[test]
    fn setup_prompts_not_empty() {
        let prompts = CaliberEcho::get_setup_prompts();
        assert!(!prompts.is_empty());
        assert_eq!(prompts[0].key, "docs_dir");
    }

    #[test]
    fn plugin_meta() {
        let caliber = CaliberEcho::new(PathBuf::from("/tmp"));
        let meta = caliber.meta();
        assert_eq!(meta.name, "caliber-echo");
    }

    #[test]
    fn plugin_role_is_outcome() {
        let caliber = CaliberEcho::new(PathBuf::from("/tmp"));
        assert_eq!(caliber.role(), PluginRole::Outcome);
    }
}
