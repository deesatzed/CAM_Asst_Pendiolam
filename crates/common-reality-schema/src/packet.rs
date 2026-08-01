use serde::Deserialize;
use std::collections::HashSet;
use std::fmt;

const PERMITTED_PROFILE_FIELDS: &[&str] = &[
    "language",
    "literacy_level",
    "technical_familiarity",
    "preferred_length",
    "modality",
    "accessibility_needs",
    "explicitly_selected_examples",
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedRealityPacket {
    pub schema_version: u8,
    pub packet_id: String,
    pub version: u64,
    pub question: String,
    pub scope: Scope,
    pub status: String,
    pub sources: Vec<Source>,
    pub claims: Vec<Claim>,
    pub evidence_objects: Vec<Evidence>,
    pub interpretations: Vec<Interpretation>,
    pub value_positions: Vec<ValuePosition>,
    pub action_options: Vec<ActionOption>,
    pub unknowns: Vec<String>,
    pub minority_reports: Vec<MinorityReport>,
    pub transformation_contract: TransformationContract,
    pub provenance: Provenance,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    pub description: String,
    pub valid_at: String,
    pub jurisdictions: Vec<String>,
    pub populations: Vec<String>,
    pub exclusions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub source_id: String,
    pub title: String,
    pub content_digest: String,
    pub source_type: String,
    pub retrieved_at: String,
    pub issuer: Option<String>,
    pub locator: Option<String>,
    pub quality_notes: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Claim {
    pub claim_id: String,
    pub proposition: String,
    pub claim_type: String,
    pub status: String,
    pub evidence_for: Vec<String>,
    pub evidence_against: Vec<String>,
    pub applicability: String,
    pub uncertainty: Uncertainty,
    pub materiality: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Uncertainty {
    pub level: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub evidence_id: String,
    pub source_refs: Vec<String>,
    pub summary: String,
    pub direction: String,
    pub limitations: Vec<String>,
    pub confidence: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Interpretation {
    pub interpretation_id: String,
    pub description: String,
    pub claim_refs: Vec<String>,
    pub assumptions: Vec<String>,
    pub counterarguments: Vec<String>,
    pub distinguishing_tests: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValuePosition {
    pub value_id: String,
    pub statement: String,
    pub holder_scope: String,
    pub tensions_with: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionOption {
    pub option_id: String,
    pub action: String,
    pub assumptions: Vec<String>,
    pub affected_groups: Vec<String>,
    pub risks: Vec<String>,
    pub reversibility: String,
    pub success_measures: Vec<String>,
    pub stop_conditions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinorityReport {
    pub report_id: String,
    pub position: String,
    pub evidence_refs: Vec<String>,
    pub unresolved_risk: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransformationContract {
    pub allowed_profile_fields: Vec<String>,
    pub allowed_transformations: Vec<String>,
    pub forbidden_transformations: Vec<String>,
    pub required_materiality_levels: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub manifest_digest: String,
    pub object_root: String,
    pub created_at: String,
    pub authors: Vec<String>,
    pub reviewers: Vec<String>,
    pub signatures: Vec<String>,
    pub transparency_receipts: Vec<String>,
    pub supersedes: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PacketValidationError(String);

impl fmt::Display for PacketValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for PacketValidationError {}

pub fn validate_packet(json: &str) -> Result<SharedRealityPacket, PacketValidationError> {
    let packet: SharedRealityPacket = serde_json::from_str(json)
        .map_err(|error| PacketValidationError(format!("invalid packet JSON: {error}")))?;

    if packet.schema_version != 1 || packet.packet_id.is_empty() || packet.version == 0 {
        return Err(PacketValidationError("invalid packet identity".to_owned()));
    }
    if packet.question.is_empty()
        || packet.scope.description.is_empty()
        || packet.scope.valid_at.is_empty()
    {
        return Err(PacketValidationError(
            "missing packet question or scope".to_owned(),
        ));
    }
    if !matches!(
        packet.status.as_str(),
        "draft" | "source_frozen" | "reviewed" | "published"
    ) {
        return Err(PacketValidationError(
            "unsupported packet status".to_owned(),
        ));
    }
    if packet.unknowns.is_empty() {
        return Err(PacketValidationError(
            "packet must declare unknowns".to_owned(),
        ));
    }

    let source_ids = unique_ids(
        packet
            .sources
            .iter()
            .map(|source| source.source_id.as_str()),
        "source",
    )?;
    for source in &packet.sources {
        if source.title.is_empty()
            || source.content_digest.len() < 8
            || source.source_type.is_empty()
            || source.retrieved_at.is_empty()
            || source.quality_notes.is_empty()
        {
            return Err(PacketValidationError("incomplete source object".to_owned()));
        }
    }

    let evidence_ids = unique_ids(
        packet
            .evidence_objects
            .iter()
            .map(|evidence| evidence.evidence_id.as_str()),
        "evidence",
    )?;
    for evidence in &packet.evidence_objects {
        if evidence.source_refs.is_empty()
            || evidence.summary.is_empty()
            || !matches!(
                evidence.direction.as_str(),
                "supports" | "challenges" | "contextualizes" | "neutral"
            )
            || !matches!(
                evidence.confidence.as_str(),
                "low" | "medium" | "high" | "not_scored"
            )
        {
            return Err(PacketValidationError(
                "incomplete evidence object".to_owned(),
            ));
        }
        ensure_known_references(&evidence.source_refs, &source_ids, "evidence source")?;
    }

    let claim_ids = unique_ids(
        packet.claims.iter().map(|claim| claim.claim_id.as_str()),
        "claim",
    )?;
    for claim in &packet.claims {
        if claim.proposition.is_empty()
            || !matches!(
                claim.claim_type.as_str(),
                "observation"
                    | "measurement"
                    | "event"
                    | "association"
                    | "generalization"
                    | "forecast"
            )
            || !matches!(
                claim.status.as_str(),
                "supported" | "contested" | "uncertain" | "unsupported" | "outdated" | "refuted"
            )
            || !matches!(
                claim.uncertainty.level.as_str(),
                "low" | "moderate" | "high" | "unknown"
            )
            || claim.uncertainty.reasons.is_empty()
        {
            return Err(PacketValidationError("incomplete claim object".to_owned()));
        }
        ensure_known_references(&claim.evidence_for, &evidence_ids, "claim evidence")?;
        ensure_known_references(
            &claim.evidence_against,
            &evidence_ids,
            "claim counterevidence",
        )?;
        if matches!(claim.materiality.as_str(), "high" | "critical")
            && claim.evidence_against.is_empty()
        {
            return Err(PacketValidationError(
                "material claim lacks counterevidence disclosure".to_owned(),
            ));
        }
    }

    for interpretation in &packet.interpretations {
        if interpretation.interpretation_id.is_empty()
            || interpretation.description.is_empty()
            || interpretation.claim_refs.is_empty()
            || interpretation.assumptions.is_empty()
        {
            return Err(PacketValidationError(
                "incomplete interpretation object".to_owned(),
            ));
        }
        ensure_known_references(
            &interpretation.claim_refs,
            &claim_ids,
            "interpretation claim",
        )?;
    }
    for value in &packet.value_positions {
        if value.value_id.is_empty() || value.statement.is_empty() || value.holder_scope.is_empty()
        {
            return Err(PacketValidationError(
                "incomplete value position".to_owned(),
            ));
        }
    }
    for action in &packet.action_options {
        if action.option_id.is_empty()
            || action.action.is_empty()
            || action.assumptions.is_empty()
            || action.risks.is_empty()
            || !matches!(
                action.reversibility.as_str(),
                "high" | "moderate" | "low" | "irreversible"
            )
        {
            return Err(PacketValidationError("incomplete action option".to_owned()));
        }
    }
    for report in &packet.minority_reports {
        if report.report_id.is_empty()
            || report.position.is_empty()
            || report.unresolved_risk.is_empty()
        {
            return Err(PacketValidationError(
                "incomplete minority report".to_owned(),
            ));
        }
        ensure_known_references(&report.evidence_refs, &evidence_ids, "minority evidence")?;
    }

    let allowed_profile_fields: HashSet<&str> = packet
        .transformation_contract
        .allowed_profile_fields
        .iter()
        .map(String::as_str)
        .collect();
    if allowed_profile_fields.len() != packet.transformation_contract.allowed_profile_fields.len()
        || allowed_profile_fields
            .iter()
            .any(|field| !PERMITTED_PROFILE_FIELDS.contains(field))
        || packet
            .transformation_contract
            .allowed_transformations
            .is_empty()
        || packet
            .transformation_contract
            .forbidden_transformations
            .is_empty()
    {
        return Err(PacketValidationError(
            "invalid Lens transformation contract".to_owned(),
        ));
    }
    if packet.provenance.manifest_digest.len() < 8
        || packet.provenance.object_root.len() < 8
        || packet.provenance.created_at.is_empty()
        || packet.provenance.authors.is_empty()
        || packet.provenance.reviewers.is_empty()
    {
        return Err(PacketValidationError("incomplete provenance".to_owned()));
    }

    Ok(packet)
}

fn unique_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    kind: &str,
) -> Result<HashSet<&'a str>, PacketValidationError> {
    let ids: HashSet<&str> = ids.collect();
    if ids.is_empty() || ids.iter().any(|id| id.is_empty()) {
        return Err(PacketValidationError(format!("missing {kind} identifier")));
    }
    Ok(ids)
}

fn ensure_known_references(
    references: &[String],
    known_ids: &HashSet<&str>,
    kind: &str,
) -> Result<(), PacketValidationError> {
    if references
        .iter()
        .any(|reference| !known_ids.contains(reference.as_str()))
    {
        return Err(PacketValidationError(format!("unknown {kind} reference")));
    }
    Ok(())
}
