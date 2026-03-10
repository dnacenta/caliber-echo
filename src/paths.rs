//! Path resolution for caliber-echo.

use std::path::PathBuf;

/// Resolve the entity documents directory.
/// Checks CALIBER_ECHO_DOCS env var first, then falls back to home directory.
pub fn docs_dir() -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("CALIBER_ECHO_DOCS") {
        return Ok(PathBuf::from(p));
    }
    dirs::home_dir().ok_or_else(|| "Could not determine home directory".to_string())
}

/// Resolve the caliber data directory (where outcomes.json lives).
/// This is `{docs_dir}/caliber/`.
pub fn caliber_dir(docs_dir: &std::path::Path) -> PathBuf {
    docs_dir.join("caliber")
}

/// Path to the outcomes file.
pub fn outcomes_file(docs_dir: &std::path::Path) -> PathBuf {
    caliber_dir(docs_dir).join("outcomes.json")
}

/// Path to CALIBER.md.
pub fn caliber_md(docs_dir: &std::path::Path) -> PathBuf {
    docs_dir.join("CALIBER.md")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn caliber_dir_is_under_docs() {
        let docs = Path::new("/tmp/entity");
        assert_eq!(caliber_dir(docs), Path::new("/tmp/entity/caliber"));
    }

    #[test]
    fn outcomes_file_is_under_caliber() {
        let docs = Path::new("/tmp/entity");
        assert_eq!(
            outcomes_file(docs),
            Path::new("/tmp/entity/caliber/outcomes.json")
        );
    }

    #[test]
    fn caliber_md_is_at_root() {
        let docs = Path::new("/tmp/entity");
        assert_eq!(caliber_md(docs), Path::new("/tmp/entity/CALIBER.md"));
    }
}
