use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

mod packet;

pub use packet::{
    validate_packet, ActionOption, Claim, Evidence, Interpretation, MinorityReport,
    PacketValidationError, Provenance, Scope, SharedRealityPacket, Source, TransformationContract,
    Uncertainty, ValuePosition,
};

pub const REQUIRED_ATTACK_FIXTURE_IDS: &[&str] = &[
    "unsupported_addition",
    "material_omission",
    "uncertainty_reduction",
    "interpretation_as_fact",
    "value_as_evidence",
    "forbidden_profile_use",
    "stakeholder_omission",
    "source_laundering",
    "stale_packet",
    "minority_deletion",
    "model_collusion",
    "prompt_injection",
    "packet_truncation",
    "false_transparency_receipt",
];

#[derive(Debug, Deserialize)]
pub struct FixtureManifest {
    schema_version: u8,
    corpus_id: String,
    fixtures: Vec<FixtureEntry>,
}

impl FixtureManifest {
    pub fn fixture_ids(&self) -> Vec<String> {
        self.fixtures
            .iter()
            .map(|fixture| fixture.id.clone())
            .collect()
    }
}

#[derive(Debug, Deserialize)]
struct FixtureEntry {
    id: String,
    path: String,
    sha256: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FixtureManifestError(String);

impl fmt::Display for FixtureManifestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for FixtureManifestError {}

/// Returns the repository-owned Gate 0 fixture manifest location.
pub fn fixture_manifest_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/campus-ai-v1/manifest.json")
}

pub fn verify_fixture_manifest() -> Result<FixtureManifest, FixtureManifestError> {
    let manifest_path = fixture_manifest_path();
    let manifest_bytes = fs::read(&manifest_path)
        .map_err(|error| FixtureManifestError(format!("cannot read fixture manifest: {error}")))?;
    let manifest: FixtureManifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| FixtureManifestError(format!("invalid fixture manifest JSON: {error}")))?;

    if manifest.schema_version != 1 {
        return Err(FixtureManifestError(
            "unsupported fixture manifest schema version".to_owned(),
        ));
    }
    if manifest.corpus_id != "campus-ai-v1" {
        return Err(FixtureManifestError(
            "unexpected fixture corpus identifier".to_owned(),
        ));
    }

    let fixture_ids: HashSet<&str> = manifest
        .fixtures
        .iter()
        .map(|fixture| fixture.id.as_str())
        .collect();
    if fixture_ids.len() != manifest.fixtures.len() {
        return Err(FixtureManifestError(
            "duplicate fixture identifier".to_owned(),
        ));
    }
    if !fixture_ids.contains("packet") {
        return Err(FixtureManifestError(
            "missing canonical packet fixture".to_owned(),
        ));
    }
    for attack_id in REQUIRED_ATTACK_FIXTURE_IDS {
        if !fixture_ids.contains(attack_id) {
            return Err(FixtureManifestError(format!(
                "missing required attack fixture: {attack_id}"
            )));
        }
    }

    let fixture_root = manifest_path.parent().ok_or_else(|| {
        FixtureManifestError("fixture manifest has no parent directory".to_owned())
    })?;
    for fixture in &manifest.fixtures {
        let relative_path = Path::new(&fixture.path);
        if relative_path.is_absolute()
            || relative_path.components().any(|component| {
                matches!(
                    component,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            })
        {
            return Err(FixtureManifestError(format!(
                "unsafe fixture path: {}",
                fixture.path
            )));
        }

        let bytes = fs::read(fixture_root.join(relative_path)).map_err(|error| {
            FixtureManifestError(format!("cannot read fixture {}: {error}", fixture.id))
        })?;
        let actual_hash = format!("{:x}", Sha256::digest(bytes));
        if actual_hash != fixture.sha256 {
            return Err(FixtureManifestError(format!(
                "fixture digest mismatch: {}",
                fixture.id
            )));
        }
    }

    Ok(manifest)
}
