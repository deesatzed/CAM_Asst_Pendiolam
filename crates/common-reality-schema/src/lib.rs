use std::path::PathBuf;

/// Returns the repository-owned Gate 0 fixture manifest location.
pub fn fixture_manifest_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/campus-ai-v1/manifest.json")
}

