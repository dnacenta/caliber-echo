//! Path resolution for pulse-echo.

use std::path::PathBuf;

/// Resolve the entity documents directory.
/// Checks PULSE_ECHO_DOCS env var first, then falls back to home directory.
pub fn docs_dir() -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("PULSE_ECHO_DOCS") {
        return Ok(PathBuf::from(p));
    }
    dirs::home_dir().ok_or_else(|| "Could not determine home directory".to_string())
}

/// Resolve the pulse data directory (where outcomes.json lives).
/// This is `{docs_dir}/pulse/`.
pub fn pulse_dir(docs_dir: &std::path::Path) -> PathBuf {
    docs_dir.join("pulse")
}

/// Path to the outcomes file.
pub fn outcomes_file(docs_dir: &std::path::Path) -> PathBuf {
    pulse_dir(docs_dir).join("outcomes.json")
}

/// Path to PULSE.md.
pub fn pulse_md(docs_dir: &std::path::Path) -> PathBuf {
    docs_dir.join("PULSE.md")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn pulse_dir_is_under_docs() {
        let docs = Path::new("/tmp/entity");
        assert_eq!(pulse_dir(docs), Path::new("/tmp/entity/pulse"));
    }

    #[test]
    fn outcomes_file_is_under_pulse() {
        let docs = Path::new("/tmp/entity");
        assert_eq!(
            outcomes_file(docs),
            Path::new("/tmp/entity/pulse/outcomes.json")
        );
    }

    #[test]
    fn pulse_md_is_at_root() {
        let docs = Path::new("/tmp/entity");
        assert_eq!(pulse_md(docs), Path::new("/tmp/entity/PULSE.md"));
    }
}
