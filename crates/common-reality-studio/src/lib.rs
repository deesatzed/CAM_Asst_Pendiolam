use common_reality_canonical::{canonical_json_value, content_id_bytes, content_id_value};
use common_reality_ledger::{Ledger, OperationState};
use common_reality_policy::{authorize, AuthorizationRequest, CapabilityGrant, Decision};
use common_reality_schema::validate_packet;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub const PUBLICATION_PURPOSE: &str = "common-reality-gate2";
pub const REVIEW_ACTION: &str = "review_packet";
pub const PUBLISH_ACTION: &str = "publish_packet";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceMetadata {
    pub source_id: String,
    pub title: String,
    pub source_type: String,
    pub origin: String,
    pub retrieved_at: String,
    pub issuer: Option<String>,
    pub locator: Option<String>,
    pub quality_notes: String,
    pub limitations: Vec<String>,
}

impl SourceMetadata {
    pub fn synthetic(
        source_id: impl Into<String>,
        title: impl Into<String>,
        origin: impl Into<String>,
        retrieved_at: impl Into<String>,
    ) -> Self {
        Self {
            source_id: source_id.into(),
            title: title.into(),
            source_type: "synthetic".into(),
            origin: origin.into(),
            retrieved_at: retrieved_at.into(),
            issuer: Some("Common Reality synthetic fixture".into()),
            locator: None,
            quality_notes: "Synthetic local evaluation source.".into(),
            limitations: vec!["Not a real-world source.".into()],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceObject {
    pub source_id: String,
    pub title: String,
    pub source_type: String,
    pub origin: String,
    pub retrieved_at: String,
    pub issuer: Option<String>,
    pub locator: Option<String>,
    pub quality_notes: String,
    pub limitations: Vec<String>,
    pub content_digest: String,
    pub object_id: String,
    pub byte_length: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimObject {
    pub claim_id: String,
    pub proposition: String,
    pub claim_type: String,
    pub support_status: String,
    pub evidence_refs: Vec<String>,
    pub counterevidence_refs: Vec<String>,
    pub uncertainty_ref: String,
    pub applicability: String,
    pub materiality: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceObject {
    pub evidence_id: String,
    pub source_refs: Vec<String>,
    pub summary: String,
    pub direction: String,
    pub limitations: Vec<String>,
    pub confidence: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CounterevidenceObject {
    pub counterevidence_id: String,
    pub claim_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub statement: String,
    pub materiality: String,
    pub visible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UncertaintyObject {
    pub uncertainty_id: String,
    pub claim_refs: Vec<String>,
    pub level: String,
    pub reasons: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterpretationObject {
    pub interpretation_id: String,
    pub description: String,
    pub claim_refs: Vec<String>,
    pub assumptions: Vec<String>,
    pub counterarguments: Vec<String>,
    pub distinguishing_tests: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValueObject {
    pub value_id: String,
    pub statement: String,
    pub holder_scope: String,
    pub tensions_with: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionObject {
    pub option_id: String,
    pub action: String,
    pub assumptions: Vec<String>,
    pub affected_groups: Vec<String>,
    pub risks: Vec<String>,
    pub reversibility: String,
    pub success_measures: Vec<String>,
    pub stop_conditions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinorityReportObject {
    pub report_id: String,
    pub position: String,
    pub evidence_refs: Vec<String>,
    pub unresolved_risk: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeObject {
    pub description: String,
    pub valid_at: String,
    pub jurisdictions: Vec<String>,
    pub populations: Vec<String>,
    pub exclusions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransformationContractObject {
    pub allowed_profile_fields: Vec<String>,
    pub allowed_transformations: Vec<String>,
    pub forbidden_transformations: Vec<String>,
    pub required_materiality_levels: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DraftProvenance {
    pub created_at: String,
    pub authors: Vec<String>,
    pub reviewers: Vec<String>,
    pub origin_label: String,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "source_frozen")]
    SourceFrozen,
    #[serde(rename = "reviewed")]
    Reviewed,
    #[serde(rename = "published")]
    Published,
}

impl PacketStatus {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::SourceFrozen => "source_frozen",
            Self::Reviewed => "reviewed",
            Self::Published => "published",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "object_type", content = "data")]
pub enum TypedObject {
    #[serde(rename = "source")]
    Source(SourceObject),
    #[serde(rename = "claim")]
    Claim(ClaimObject),
    #[serde(rename = "evidence")]
    Evidence(EvidenceObject),
    #[serde(rename = "counterevidence")]
    Counterevidence(CounterevidenceObject),
    #[serde(rename = "uncertainty")]
    Uncertainty(UncertaintyObject),
    #[serde(rename = "interpretation")]
    Interpretation(InterpretationObject),
    #[serde(rename = "value")]
    Value(ValueObject),
    #[serde(rename = "action")]
    Action(ActionObject),
    #[serde(rename = "minority_report")]
    MinorityReport(MinorityReportObject),
}

impl TypedObject {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Source(_) => "source",
            Self::Claim(_) => "claim",
            Self::Evidence(_) => "evidence",
            Self::Counterevidence(_) => "counterevidence",
            Self::Uncertainty(_) => "uncertainty",
            Self::Interpretation(_) => "interpretation",
            Self::Value(_) => "value",
            Self::Action(_) => "action",
            Self::MinorityReport(_) => "minority_report",
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Source(value) => &value.source_id,
            Self::Claim(value) => &value.claim_id,
            Self::Evidence(value) => &value.evidence_id,
            Self::Counterevidence(value) => &value.counterevidence_id,
            Self::Uncertainty(value) => &value.uncertainty_id,
            Self::Interpretation(value) => &value.interpretation_id,
            Self::Value(value) => &value.value_id,
            Self::Action(value) => &value.option_id,
            Self::MinorityReport(value) => &value.report_id,
        }
    }

    fn canonical_bytes(&self) -> Result<Vec<u8>, StudioError> {
        let value = serde_json::to_value(self).map_err(StudioError::serialization)?;
        Ok(canonical_json_value(&value))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PacketStudioDraft {
    pub packet_id: String,
    pub version: u64,
    pub parent_version: Option<u64>,
    pub question: String,
    pub scope: ScopeObject,
    pub status: PacketStatus,
    pub objects: Vec<TypedObject>,
    pub unknowns: Vec<String>,
    pub transformation_contract: TransformationContractObject,
    pub provenance: DraftProvenance,
}

impl PacketStudioDraft {
    pub fn from_legacy_packet(packet_json: &str) -> Result<Self, StudioError> {
        validate_packet(packet_json)
            .map_err(|error| StudioError::InvalidPacket(error.to_string()))?;
        let packet: Value = serde_json::from_str(packet_json).map_err(StudioError::json)?;
        let scope_value = packet
            .get("scope")
            .ok_or_else(|| StudioError::InvalidPacket("missing scope".into()))?;
        let scope = ScopeObject {
            description: required_string(scope_value, "description")?,
            valid_at: required_string(scope_value, "valid_at")?,
            jurisdictions: strings(scope_value, "jurisdictions")?,
            populations: strings(scope_value, "populations")?,
            exclusions: strings(scope_value, "exclusions")?,
        };

        let mut objects = Vec::new();
        for source in packet
            .get("sources")
            .and_then(Value::as_array)
            .ok_or_else(|| StudioError::InvalidPacket("missing sources".into()))?
        {
            objects.push(TypedObject::Source(SourceObject {
                source_id: required_string(source, "source_id")?,
                title: required_string(source, "title")?,
                source_type: required_string(source, "source_type")?,
                origin: "legacy-fixture".into(),
                retrieved_at: required_string(source, "retrieved_at")?,
                issuer: optional_string(source, "issuer"),
                locator: optional_string(source, "locator"),
                quality_notes: required_string(source, "quality_notes")?,
                limitations: strings(source, "limitations")?,
                content_digest: required_string(source, "content_digest")?,
                object_id: required_string(source, "content_digest")?,
                byte_length: 0,
            }));
        }

        let claims = packet
            .get("claims")
            .and_then(Value::as_array)
            .ok_or_else(|| StudioError::InvalidPacket("missing claims".into()))?;
        for claim in claims {
            let claim_id = required_string(claim, "claim_id")?;
            let uncertainty_id = format!("uncertainty:{claim_id}");
            objects.push(TypedObject::Claim(ClaimObject {
                claim_id: claim_id.clone(),
                proposition: required_string(claim, "proposition")?,
                claim_type: required_string(claim, "claim_type")?,
                support_status: required_string(claim, "status")?,
                evidence_refs: strings(claim, "evidence_for")?,
                counterevidence_refs: strings(claim, "evidence_against")?,
                uncertainty_ref: uncertainty_id.clone(),
                applicability: required_string(claim, "applicability")?,
                materiality: required_string(claim, "materiality")?,
            }));
            let uncertainty = claim
                .get("uncertainty")
                .ok_or_else(|| StudioError::InvalidPacket("missing uncertainty".into()))?;
            objects.push(TypedObject::Uncertainty(UncertaintyObject {
                uncertainty_id,
                claim_refs: vec![claim_id.clone()],
                level: required_string(uncertainty, "level")?,
                reasons: strings(uncertainty, "reasons")?,
            }));
            let counter_refs = strings(claim, "evidence_against")?;
            if !counter_refs.is_empty() {
                objects.push(TypedObject::Counterevidence(CounterevidenceObject {
                    counterevidence_id: format!("counterevidence:{claim_id}"),
                    claim_refs: vec![claim_id],
                    evidence_refs: counter_refs,
                    statement: "Material counterevidence is disclosed in the packet.".into(),
                    materiality: required_string(claim, "materiality")?,
                    visible: true,
                }));
            }
        }

        for evidence in packet
            .get("evidence_objects")
            .and_then(Value::as_array)
            .ok_or_else(|| StudioError::InvalidPacket("missing evidence".into()))?
        {
            objects.push(TypedObject::Evidence(EvidenceObject {
                evidence_id: required_string(evidence, "evidence_id")?,
                source_refs: strings(evidence, "source_refs")?,
                summary: required_string(evidence, "summary")?,
                direction: required_string(evidence, "direction")?,
                limitations: strings(evidence, "limitations")?,
                confidence: required_string(evidence, "confidence")?,
            }));
        }

        for interpretation in array(packet.get("interpretations"))? {
            objects.push(TypedObject::Interpretation(InterpretationObject {
                interpretation_id: required_string(interpretation, "interpretation_id")?,
                description: required_string(interpretation, "description")?,
                claim_refs: strings(interpretation, "claim_refs")?,
                assumptions: strings(interpretation, "assumptions")?,
                counterarguments: strings(interpretation, "counterarguments")?,
                distinguishing_tests: strings(interpretation, "distinguishing_tests")?,
            }));
        }
        for value in array(packet.get("value_positions"))? {
            objects.push(TypedObject::Value(ValueObject {
                value_id: required_string(value, "value_id")?,
                statement: required_string(value, "statement")?,
                holder_scope: required_string(value, "holder_scope")?,
                tensions_with: strings(value, "tensions_with")?,
            }));
        }
        for action in array(packet.get("action_options"))? {
            objects.push(TypedObject::Action(ActionObject {
                option_id: required_string(action, "option_id")?,
                action: required_string(action, "action")?,
                assumptions: strings(action, "assumptions")?,
                affected_groups: strings(action, "affected_groups")?,
                risks: strings(action, "risks")?,
                reversibility: required_string(action, "reversibility")?,
                success_measures: strings(action, "success_measures")?,
                stop_conditions: strings(action, "stop_conditions")?,
            }));
        }
        for report in array(packet.get("minority_reports"))? {
            objects.push(TypedObject::MinorityReport(MinorityReportObject {
                report_id: required_string(report, "report_id")?,
                position: required_string(report, "position")?,
                evidence_refs: strings(report, "evidence_refs")?,
                unresolved_risk: required_string(report, "unresolved_risk")?,
            }));
        }

        let transformation = packet
            .get("transformation_contract")
            .ok_or_else(|| StudioError::InvalidPacket("missing transformation contract".into()))?;
        let provenance = packet
            .get("provenance")
            .ok_or_else(|| StudioError::InvalidPacket("missing provenance".into()))?;
        Ok(Self {
            packet_id: required_string(&packet, "packet_id")?,
            version: required_u64(&packet, "version")?,
            parent_version: None,
            question: required_string(&packet, "question")?,
            scope,
            status: parse_status(required_string(&packet, "status")?.as_str())?,
            objects,
            unknowns: strings(&packet, "unknowns")?,
            transformation_contract: TransformationContractObject {
                allowed_profile_fields: strings(transformation, "allowed_profile_fields")?,
                allowed_transformations: strings(transformation, "allowed_transformations")?,
                forbidden_transformations: strings(transformation, "forbidden_transformations")?,
                required_materiality_levels: strings(
                    transformation,
                    "required_materiality_levels",
                )?,
            },
            provenance: DraftProvenance {
                created_at: required_string(provenance, "created_at")?,
                authors: strings(provenance, "authors")?,
                reviewers: strings(provenance, "reviewers")?,
                origin_label: "synthetic-campus-ai-v1".into(),
                limitations: vec!["Fixture-backed draft; not a real policy.".into()],
            },
        })
    }

    pub fn objects(&self) -> &[TypedObject] {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut Vec<TypedObject> {
        &mut self.objects
    }

    pub fn claim_count(&self) -> usize {
        self.objects
            .iter()
            .filter(|object| matches!(object, TypedObject::Claim(_)))
            .count()
    }

    pub fn counterevidence_count(&self) -> usize {
        self.objects
            .iter()
            .filter(|object| matches!(object, TypedObject::Counterevidence(_)))
            .count()
    }

    pub fn uncertainty_count(&self) -> usize {
        self.objects
            .iter()
            .filter(|object| matches!(object, TypedObject::Uncertainty(_)))
            .count()
    }

    pub fn find_object_mut(&mut self, id: &str) -> Option<&mut TypedObject> {
        self.objects.iter_mut().find(|object| object.id() == id)
    }

    pub fn remove_object(&mut self, id: &str) -> bool {
        let before = self.objects.len();
        self.objects.retain(|object| object.id() != id);
        before != self.objects.len()
    }

    pub fn next_version(&self) -> Self {
        let mut next = self.clone();
        next.version = self.version + 1;
        next.parent_version = Some(self.version);
        next.status = PacketStatus::Draft;
        next
    }

    pub fn state_digest(&self) -> Result<String, StudioError> {
        let value = serde_json::to_value(self).map_err(StudioError::serialization)?;
        Ok(content_id_value(&value))
    }

    pub fn legacy_packet_json(&self) -> Result<String, StudioError> {
        let value = self.legacy_packet_value()?;
        serde_json::to_string(&value).map_err(StudioError::serialization)
    }

    fn legacy_packet_value(&self) -> Result<Value, StudioError> {
        let mut sources = Vec::new();
        let mut claims = Vec::new();
        let mut evidence_objects = Vec::new();
        let mut interpretations = Vec::new();
        let mut value_positions = Vec::new();
        let mut action_options = Vec::new();
        let mut minority_reports = Vec::new();
        let uncertainties: BTreeMap<String, UncertaintyObject> = self
            .objects
            .iter()
            .filter_map(|object| match object {
                TypedObject::Uncertainty(value) => {
                    Some((value.uncertainty_id.clone(), value.clone()))
                }
                _ => None,
            })
            .collect();

        for object in &self.objects {
            match object {
                TypedObject::Source(source) => sources.push(json!({
                    "source_id": source.source_id,
                    "title": source.title,
                    "content_digest": source.content_digest,
                    "source_type": source.source_type,
                    "retrieved_at": source.retrieved_at,
                    "issuer": source.issuer,
                    "locator": source.locator,
                    "quality_notes": source.quality_notes,
                    "limitations": source.limitations,
                })),
                TypedObject::Claim(claim) => {
                    let uncertainty =
                        uncertainties.get(&claim.uncertainty_ref).ok_or_else(|| {
                            StudioError::InvalidPacket(format!(
                                "claim {} references missing uncertainty {}",
                                claim.claim_id, claim.uncertainty_ref
                            ))
                        })?;
                    claims.push(json!({
                        "claim_id": claim.claim_id,
                        "proposition": claim.proposition,
                        "claim_type": claim.claim_type,
                        "status": claim.support_status,
                        "evidence_for": claim.evidence_refs,
                        "evidence_against": claim.counterevidence_refs,
                        "applicability": claim.applicability,
                        "uncertainty": {
                            "level": uncertainty.level,
                            "reasons": uncertainty.reasons,
                        },
                        "materiality": claim.materiality,
                    }));
                }
                TypedObject::Evidence(evidence) => evidence_objects.push(json!({
                    "evidence_id": evidence.evidence_id,
                    "source_refs": evidence.source_refs,
                    "summary": evidence.summary,
                    "direction": evidence.direction,
                    "limitations": evidence.limitations,
                    "confidence": evidence.confidence,
                })),
                TypedObject::Interpretation(value) => interpretations.push(json!({
                    "interpretation_id": value.interpretation_id,
                    "description": value.description,
                    "claim_refs": value.claim_refs,
                    "assumptions": value.assumptions,
                    "counterarguments": value.counterarguments,
                    "distinguishing_tests": value.distinguishing_tests,
                })),
                TypedObject::Value(value) => value_positions.push(json!({
                    "value_id": value.value_id,
                    "statement": value.statement,
                    "holder_scope": value.holder_scope,
                    "tensions_with": value.tensions_with,
                })),
                TypedObject::Action(value) => action_options.push(json!({
                    "option_id": value.option_id,
                    "action": value.action,
                    "assumptions": value.assumptions,
                    "affected_groups": value.affected_groups,
                    "risks": value.risks,
                    "reversibility": value.reversibility,
                    "success_measures": value.success_measures,
                    "stop_conditions": value.stop_conditions,
                })),
                TypedObject::MinorityReport(value) => minority_reports.push(json!({
                    "report_id": value.report_id,
                    "position": value.position,
                    "evidence_refs": value.evidence_refs,
                    "unresolved_risk": value.unresolved_risk,
                })),
                TypedObject::Counterevidence(_) | TypedObject::Uncertainty(_) => {}
            }
        }

        Ok(json!({
            "schema_version": 1,
            "packet_id": self.packet_id,
            "version": self.version,
            "question": self.question,
            "scope": self.scope,
            "status": self.status.as_str(),
            "sources": sources,
            "claims": claims,
            "evidence_objects": evidence_objects,
            "interpretations": interpretations,
            "value_positions": value_positions,
            "action_options": action_options,
            "unknowns": self.unknowns,
            "minority_reports": minority_reports,
            "transformation_contract": self.transformation_contract,
            "provenance": {
                "manifest_digest": "gate2-local-manifest",
                "object_root": "gate2-local-object-root",
                "created_at": self.provenance.created_at,
                "authors": self.provenance.authors,
                "reviewers": self.provenance.reviewers,
                "signatures": [],
                "transparency_receipts": [],
                "supersedes": self.parent_version.map(|version| format!("{}:{version}", self.packet_id)),
            },
        }))
    }

    pub fn review(&self) -> ReviewReport {
        let mut findings = Vec::new();
        let mut ids: BTreeMap<String, &'static str> = BTreeMap::new();
        let mut known: BTreeMap<&'static str, BTreeSet<String>> = BTreeMap::new();
        for object in &self.objects {
            let kind = object.kind();
            if ids.insert(object.id().to_owned(), kind).is_some() {
                findings.push(ReviewFinding::blocker(
                    "duplicate_object_id",
                    Some(object.id()),
                    "Object identifiers must be unique across the typed packet.",
                ));
            }
            known
                .entry(kind)
                .or_default()
                .insert(object.id().to_owned());
        }
        if self.packet_id.is_empty() || self.version == 0 || self.question.is_empty() {
            findings.push(ReviewFinding::blocker(
                "incomplete_packet_identity",
                None,
                "Packet identity, version, and question are required.",
            ));
        }

        let has_kind =
            |kind: &str, id: &str| known.get(kind).is_some_and(|values| values.contains(id));
        let mut visible_counterevidence_for: BTreeSet<String> = BTreeSet::new();
        for object in &self.objects {
            if let TypedObject::Counterevidence(counterevidence) = object {
                if counterevidence.visible {
                    visible_counterevidence_for.extend(counterevidence.claim_refs.iter().cloned());
                }
            }
        }
        for object in &self.objects {
            match object {
                TypedObject::Source(source) => {
                    if source.source_id.is_empty()
                        || source.title.is_empty()
                        || source.content_digest.is_empty()
                        || source.origin.is_empty()
                        || source.retrieved_at.is_empty()
                        || source.quality_notes.is_empty()
                    {
                        findings.push(ReviewFinding::blocker(
                            "incomplete_source",
                            Some(source.source_id.as_str()),
                            "Source provenance requires origin, digest, retrieval time, and limitations.",
                        ));
                    }
                }
                TypedObject::Evidence(evidence) => {
                    for source_id in &evidence.source_refs {
                        if !has_kind("source", source_id) {
                            findings.push(ReviewFinding::blocker(
                                "invalid_reference",
                                Some(evidence.evidence_id.as_str()),
                                format!("Evidence references unknown source {source_id}."),
                            ));
                        }
                    }
                    if evidence.source_refs.is_empty() || evidence.summary.is_empty() {
                        findings.push(ReviewFinding::blocker(
                            "incomplete_evidence",
                            Some(evidence.evidence_id.as_str()),
                            "Evidence requires a source reference and summary.",
                        ));
                    }
                }
                TypedObject::Counterevidence(counterevidence) => {
                    for claim_id in &counterevidence.claim_refs {
                        if !has_kind("claim", claim_id) {
                            findings.push(ReviewFinding::blocker(
                                "invalid_reference",
                                Some(counterevidence.counterevidence_id.as_str()),
                                format!("Counterevidence references unknown claim {claim_id}."),
                            ));
                        } else if counterevidence.visible {
                            visible_counterevidence_for.insert(claim_id.clone());
                        }
                    }
                    for evidence_id in &counterevidence.evidence_refs {
                        if !has_kind("evidence", evidence_id) {
                            findings.push(ReviewFinding::blocker(
                                "invalid_reference",
                                Some(counterevidence.counterevidence_id.as_str()),
                                format!(
                                    "Counterevidence references unknown evidence {evidence_id}."
                                ),
                            ));
                        }
                    }
                    if counterevidence.claim_refs.is_empty()
                        || counterevidence.evidence_refs.is_empty()
                        || counterevidence.statement.is_empty()
                        || !counterevidence.visible
                    {
                        findings.push(ReviewFinding::blocker(
                            "incomplete_counterevidence",
                            Some(counterevidence.counterevidence_id.as_str()),
                            "Counterevidence must be typed, referenced, material, and visible.",
                        ));
                    }
                }
                TypedObject::Uncertainty(uncertainty) => {
                    for claim_id in &uncertainty.claim_refs {
                        if !has_kind("claim", claim_id) {
                            findings.push(ReviewFinding::blocker(
                                "invalid_reference",
                                Some(uncertainty.uncertainty_id.as_str()),
                                format!("Uncertainty references unknown claim {claim_id}."),
                            ));
                        }
                    }
                    if uncertainty.claim_refs.is_empty()
                        || uncertainty.level.is_empty()
                        || uncertainty.reasons.is_empty()
                    {
                        findings.push(ReviewFinding::blocker(
                            "missing_uncertainty",
                            Some(uncertainty.uncertainty_id.as_str()),
                            "Every claim uncertainty object needs a level and reason.",
                        ));
                    }
                }
                TypedObject::Claim(claim) => {
                    if !matches!(
                        claim.support_status.as_str(),
                        "supported"
                            | "contested"
                            | "uncertain"
                            | "unsupported"
                            | "outdated"
                            | "refuted"
                    ) {
                        findings.push(ReviewFinding::blocker(
                            "invalid_support_status",
                            Some(claim.claim_id.as_str()),
                            "Claim support status is not in the closed status set.",
                        ));
                    }
                    if claim.evidence_refs.is_empty() && claim.counterevidence_refs.is_empty() {
                        findings.push(ReviewFinding::blocker(
                            "missing_evidence_references",
                            Some(claim.claim_id.as_str()),
                            "Every claim requires supporting evidence or disclosed counterevidence.",
                        ));
                    }
                    if claim.support_status == "supported" && claim.evidence_refs.is_empty() {
                        findings.push(ReviewFinding::blocker(
                            "unsupported_claim_marked_supported",
                            Some(claim.claim_id.as_str()),
                            "An unsupported claim cannot be published with status supported.",
                        ));
                    }
                    for evidence_id in claim
                        .evidence_refs
                        .iter()
                        .chain(&claim.counterevidence_refs)
                    {
                        if !has_kind("evidence", evidence_id) {
                            findings.push(ReviewFinding::blocker(
                                "invalid_reference",
                                Some(claim.claim_id.as_str()),
                                format!("Claim references unknown evidence {evidence_id}."),
                            ));
                        }
                    }
                    if !has_kind("uncertainty", &claim.uncertainty_ref) {
                        findings.push(ReviewFinding::blocker(
                            "missing_uncertainty",
                            Some(claim.claim_id.as_str()),
                            format!(
                                "Claim references missing uncertainty {}.",
                                claim.uncertainty_ref
                            ),
                        ));
                    }
                    if matches!(claim.materiality.as_str(), "high" | "critical")
                        && !visible_counterevidence_for.contains(&claim.claim_id)
                    {
                        findings.push(ReviewFinding::blocker(
                            "missing_material_counterevidence",
                            Some(claim.claim_id.as_str()),
                            "Material claims require visible typed counterevidence.",
                        ));
                    }
                }
                TypedObject::Interpretation(value) => {
                    for claim_id in &value.claim_refs {
                        if !has_kind("claim", claim_id) {
                            findings.push(ReviewFinding::blocker(
                                "invalid_reference",
                                Some(value.interpretation_id.as_str()),
                                format!("Interpretation references unknown claim {claim_id}."),
                            ));
                        }
                    }
                    if value.claim_refs.is_empty() || value.assumptions.is_empty() {
                        findings.push(ReviewFinding::blocker(
                            "incomplete_interpretation",
                            Some(value.interpretation_id.as_str()),
                            "Interpretations require claim references and assumptions.",
                        ));
                    }
                }
                TypedObject::Value(value) => {
                    if value.value_id.is_empty()
                        || value.statement.is_empty()
                        || value.holder_scope.is_empty()
                    {
                        findings.push(ReviewFinding::blocker(
                            "incomplete_value",
                            Some(value.value_id.as_str()),
                            "Values remain distinct typed positions with a holder scope.",
                        ));
                    }
                }
                TypedObject::Action(action) => {
                    if action.option_id.is_empty()
                        || action.action.is_empty()
                        || action.assumptions.is_empty()
                        || action.risks.is_empty()
                        || action.stop_conditions.is_empty()
                        || action.reversibility.is_empty()
                    {
                        findings.push(ReviewFinding::blocker(
                            "incomplete_action",
                            Some(action.option_id.as_str()),
                            "Action options require assumptions, risks, reversibility, and stop conditions.",
                        ));
                    }
                }
                TypedObject::MinorityReport(report) => {
                    for evidence_id in &report.evidence_refs {
                        if !has_kind("evidence", evidence_id) {
                            findings.push(ReviewFinding::blocker(
                                "invalid_reference",
                                Some(report.report_id.as_str()),
                                format!(
                                    "Minority report references unknown evidence {evidence_id}."
                                ),
                            ));
                        }
                    }
                    if report.report_id.is_empty()
                        || report.position.is_empty()
                        || report.evidence_refs.is_empty()
                        || report.unresolved_risk.is_empty()
                    {
                        findings.push(ReviewFinding::blocker(
                            "incomplete_minority_report",
                            Some(report.report_id.as_str()),
                            "Minority reports require a position, evidence, and unresolved risk.",
                        ));
                    }
                }
            }
        }

        if let Ok(legacy) = self.legacy_packet_json() {
            if let Err(error) = validate_packet(&legacy) {
                findings.push(ReviewFinding::blocker(
                    "legacy_projection_invalid",
                    None,
                    format!("Legacy packet projection failed strict validation: {error}"),
                ));
            }
        } else {
            findings.push(ReviewFinding::blocker(
                "legacy_projection_invalid",
                None,
                "Typed packet could not be projected through the strict packet schema.",
            ));
        }
        ReviewReport::new(&self.packet_id, self.version, findings)
    }

    pub fn review_authorized(
        &self,
        grant: Option<&CapabilityGrant>,
        now: u64,
    ) -> Result<ReviewReport, StudioError> {
        authorize_action(self, REVIEW_ACTION, grant, now)?;
        Ok(self.review())
    }

    pub fn publish(
        &self,
        store: &mut ObjectStore,
        grant: Option<&CapabilityGrant>,
        now: u64,
        parent: Option<&PublishedPacket>,
    ) -> Result<PublishedPacket, StudioError> {
        authorize_action(self, PUBLISH_ACTION, grant, now)?;
        let review = self.review();
        if !review.can_publish() {
            return Err(StudioError::ReviewBlocked(review.blocking_codes()));
        }
        if store
            .published_versions
            .contains(&(self.packet_id.clone(), self.version))
        {
            return Err(StudioError::ImmutableVersion {
                packet_id: self.packet_id.clone(),
                version: self.version,
            });
        }
        match (self.version, parent) {
            (1, None) => {}
            (1, Some(_)) => {
                return Err(StudioError::InvalidParent(
                    "version one cannot have a parent".into(),
                ))
            }
            (_, None) => {
                return Err(StudioError::InvalidParent(
                    "successor publication requires a parent".into(),
                ))
            }
            (_, Some(previous)) => {
                if previous.packet_id() != self.packet_id || previous.version() + 1 != self.version
                {
                    return Err(StudioError::InvalidParent(
                        "parent must be the immediately preceding version of this packet".into(),
                    ));
                }
            }
        }

        let mut published_draft = self.clone();
        published_draft.status = PacketStatus::Published;
        let root_value =
            serde_json::to_value(&published_draft).map_err(StudioError::serialization)?;
        let root_bytes = canonical_json_value(&root_value);
        let root_object_id = content_id_bytes(&root_bytes);
        store
            .objects
            .insert(root_object_id.clone(), root_bytes.clone());

        let mut entries = vec![ManifestEntry {
            object_id: root_object_id.clone(),
            kind: "packet_draft".into(),
        }];
        for object in &published_draft.objects {
            let bytes = object.canonical_bytes()?;
            let object_id = content_id_bytes(&bytes);
            store.objects.insert(object_id.clone(), bytes);
            entries.push(ManifestEntry {
                object_id,
                kind: object.kind().into(),
            });
        }
        entries.sort_by(|left, right| {
            left.kind
                .cmp(&right.kind)
                .then(left.object_id.cmp(&right.object_id))
        });
        let mut manifest = PacketManifest {
            schema_version: 1,
            packet_id: self.packet_id.clone(),
            packet_version: self.version,
            parent_version: self.parent_version,
            root_object_id: root_object_id.clone(),
            objects: entries,
            manifest_digest: String::new(),
        };
        manifest.manifest_digest = manifest.digest()?;

        let mut ledger = Ledger::new(&self.packet_id, self.version);
        ledger
            .append(
                0,
                &self.packet_id,
                self.version,
                OperationState::SourceFrozen,
            )
            .map_err(|error| StudioError::Ledger(error.to_string()))?;
        ledger
            .append(1, &self.packet_id, self.version, OperationState::Reviewed)
            .map_err(|error| StudioError::Ledger(error.to_string()))?;
        ledger
            .append(2, &self.packet_id, self.version, OperationState::Published)
            .map_err(|error| StudioError::Ledger(error.to_string()))?;

        let object_ids = manifest
            .objects
            .iter()
            .map(|entry| entry.object_id.clone())
            .collect();
        let receipt = PublicationReceipt {
            status: "published".into(),
            packet_id: self.packet_id.clone(),
            packet_version: self.version,
            packet_content_id: root_object_id,
            manifest_digest: manifest.manifest_digest.clone(),
            object_ids,
            review_digest: review.digest.clone(),
            ledger_merkle_root: ledger.merkle_checkpoint().root,
            integrity: "local-only-integrity-receipt".into(),
            raw_source_bytes_included: false,
        };
        store
            .published_versions
            .insert((self.packet_id.clone(), self.version));
        Ok(PublishedPacket {
            packet_id: self.packet_id.clone(),
            packet_version: self.version,
            draft: published_draft,
            manifest,
            receipt,
            review,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewFinding {
    pub code: String,
    pub severity: String,
    pub object_id: Option<String>,
    pub message: String,
}

impl ReviewFinding {
    fn blocker(
        code: impl Into<String>,
        object_id: Option<&str>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            severity: "blocker".into(),
            object_id: object_id.map(str::to_owned),
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewReport {
    pub packet_id: String,
    pub packet_version: u64,
    pub findings: Vec<ReviewFinding>,
    pub digest: String,
}

impl ReviewReport {
    fn new(packet_id: &str, packet_version: u64, findings: Vec<ReviewFinding>) -> Self {
        let mut report = Self {
            packet_id: packet_id.into(),
            packet_version,
            findings,
            digest: String::new(),
        };
        let value = serde_json::to_value(&report).expect("review report is serializable");
        report.digest = content_id_value(&value);
        report
    }

    pub fn can_publish(&self) -> bool {
        self.findings
            .iter()
            .all(|finding| finding.severity != "blocker")
    }

    pub fn blocking_codes(&self) -> Vec<String> {
        self.findings
            .iter()
            .filter(|finding| finding.severity == "blocker")
            .map(|finding| finding.code.clone())
            .collect()
    }

    pub fn has_code(&self, code: &str) -> bool {
        self.findings.iter().any(|finding| finding.code == code)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestEntry {
    pub object_id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PacketManifest {
    pub schema_version: u8,
    pub packet_id: String,
    pub packet_version: u64,
    pub parent_version: Option<u64>,
    pub root_object_id: String,
    pub objects: Vec<ManifestEntry>,
    pub manifest_digest: String,
}

impl PacketManifest {
    fn digest(&self) -> Result<String, StudioError> {
        let mut value = serde_json::to_value(self).map_err(StudioError::serialization)?;
        value["manifest_digest"] = Value::String(String::new());
        Ok(content_id_value(&value))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationReceipt {
    pub status: String,
    pub packet_id: String,
    pub packet_version: u64,
    pub packet_content_id: String,
    pub manifest_digest: String,
    pub object_ids: Vec<String>,
    pub review_digest: String,
    pub ledger_merkle_root: String,
    pub integrity: String,
    pub raw_source_bytes_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublishedPacket {
    packet_id: String,
    packet_version: u64,
    pub draft: PacketStudioDraft,
    pub manifest: PacketManifest,
    pub receipt: PublicationReceipt,
    pub review: ReviewReport,
}

impl PublishedPacket {
    pub fn packet_id(&self) -> &str {
        &self.packet_id
    }

    pub fn version(&self) -> u64 {
        self.packet_version
    }

    pub fn diff_from_parent(&self, parent: &PublishedPacket) -> PacketDiff {
        PacketDiff::between(&parent.draft, &self.draft)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiffEntry {
    pub kind: String,
    pub object_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldChange {
    pub kind: String,
    pub object_id: String,
    pub field: String,
    pub before: Value,
    pub after: Value,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PacketDiff {
    pub from_version: u64,
    pub to_version: u64,
    pub additions: Vec<DiffEntry>,
    pub removals: Vec<DiffEntry>,
    pub changes: Vec<FieldChange>,
}

impl PacketDiff {
    pub fn between(before: &PacketStudioDraft, after: &PacketStudioDraft) -> Self {
        let before_map: BTreeMap<(String, String), Value> = before
            .objects
            .iter()
            .map(|object| {
                (
                    (object.kind().into(), object.id().into()),
                    serde_json::to_value(object).expect("typed object serializes"),
                )
            })
            .collect();
        let after_map: BTreeMap<(String, String), Value> = after
            .objects
            .iter()
            .map(|object| {
                (
                    (object.kind().into(), object.id().into()),
                    serde_json::to_value(object).expect("typed object serializes"),
                )
            })
            .collect();
        let mut additions = Vec::new();
        let mut removals = Vec::new();
        let mut changes = Vec::new();
        for ((kind, object_id), value) in &after_map {
            match before_map.get(&(kind.clone(), object_id.clone())) {
                None => additions.push(DiffEntry {
                    kind: kind.clone(),
                    object_id: object_id.clone(),
                }),
                Some(previous) => compare_fields(kind, object_id, previous, value, &mut changes),
            }
        }
        for (kind, object_id) in before_map.keys() {
            if !after_map.contains_key(&(kind.clone(), object_id.clone())) {
                removals.push(DiffEntry {
                    kind: kind.clone(),
                    object_id: object_id.clone(),
                });
            }
        }
        Self {
            from_version: before.version,
            to_version: after.version,
            additions,
            removals,
            changes,
        }
    }
}

fn compare_fields(
    kind: &str,
    object_id: &str,
    before: &Value,
    after: &Value,
    changes: &mut Vec<FieldChange>,
) {
    let Some(before_data) = before.get("data").and_then(Value::as_object) else {
        return;
    };
    let Some(after_data) = after.get("data").and_then(Value::as_object) else {
        return;
    };
    let keys: BTreeSet<_> = before_data.keys().chain(after_data.keys()).collect();
    for field in keys {
        let before_value = before_data.get(field).cloned().unwrap_or(Value::Null);
        let after_value = after_data.get(field).cloned().unwrap_or(Value::Null);
        if before_value != after_value {
            changes.push(FieldChange {
                kind: kind.into(),
                object_id: object_id.into(),
                field: field.clone(),
                before: before_value,
                after: after_value,
            });
        }
    }
}

#[derive(Debug)]
pub enum StudioError {
    Io(String),
    Json(String),
    Serialization(String),
    InvalidPacket(String),
    SourceTooLarge {
        path: String,
        max_bytes: u64,
        actual_bytes: u64,
    },
    MissingObject(String),
    Integrity(String),
    ReviewBlocked(Vec<String>),
    AuthorizationDenied(String),
    ImmutableVersion {
        packet_id: String,
        version: u64,
    },
    InvalidParent(String),
    Ledger(String),
}

impl StudioError {
    fn json(error: serde_json::Error) -> Self {
        Self::Json(error.to_string())
    }

    fn serialization(error: serde_json::Error) -> Self {
        Self::Serialization(error.to_string())
    }
}

impl fmt::Display for StudioError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::Json(error) => write!(formatter, "JSON error: {error}"),
            Self::Serialization(error) => write!(formatter, "serialization error: {error}"),
            Self::InvalidPacket(error) => write!(formatter, "invalid packet: {error}"),
            Self::SourceTooLarge {
                path,
                max_bytes,
                actual_bytes,
            } => write!(
                formatter,
                "source {path} is {actual_bytes} bytes, over the {max_bytes}-byte bound"
            ),
            Self::MissingObject(object_id) => write!(formatter, "missing object: {object_id}"),
            Self::Integrity(error) => write!(formatter, "integrity failure: {error}"),
            Self::ReviewBlocked(codes) => {
                write!(formatter, "review blocked publication: {}", codes.join(","))
            }
            Self::AuthorizationDenied(action) => {
                write!(formatter, "authorization denied for {action}")
            }
            Self::ImmutableVersion { packet_id, version } => write!(
                formatter,
                "published version is immutable: {packet_id}:{version}"
            ),
            Self::InvalidParent(error) => write!(formatter, "invalid parent version: {error}"),
            Self::Ledger(error) => write!(formatter, "ledger error: {error}"),
        }
    }
}

impl std::error::Error for StudioError {}

impl From<io::Error> for StudioError {
    fn from(error: io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

#[derive(Clone, Debug, Default)]
pub struct ObjectStore {
    objects: BTreeMap<String, Vec<u8>>,
    published_versions: BTreeSet<(String, u64)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains(&self, object_id: &str) -> bool {
        self.objects.contains_key(object_id)
    }

    pub fn object_bytes(&self, object_id: &str) -> Option<&[u8]> {
        self.objects.get(object_id).map(Vec::as_slice)
    }

    pub fn object_bytes_mut(&mut self, object_id: &str) -> Option<&mut Vec<u8>> {
        self.objects.get_mut(object_id)
    }

    pub fn remove_object(&mut self, object_id: &str) -> bool {
        self.objects.remove(object_id).is_some()
    }

    pub fn import_source_file(
        &mut self,
        path: &Path,
        max_bytes: u64,
        metadata: SourceMetadata,
    ) -> Result<SourceObject, StudioError> {
        let mut file = File::open(path)?;
        let actual_bytes = file.metadata()?.len();
        if actual_bytes > max_bytes {
            return Err(StudioError::SourceTooLarge {
                path: path.display().to_string(),
                max_bytes,
                actual_bytes,
            });
        }
        let mut bytes = Vec::with_capacity(actual_bytes as usize);
        file.read_to_end(&mut bytes)?;
        if bytes.len() as u64 > max_bytes {
            return Err(StudioError::SourceTooLarge {
                path: path.display().to_string(),
                max_bytes,
                actual_bytes: bytes.len() as u64,
            });
        }
        let object_id = content_id_bytes(&bytes);
        self.objects.insert(object_id.clone(), bytes.clone());
        Ok(SourceObject {
            source_id: metadata.source_id,
            title: metadata.title,
            source_type: metadata.source_type,
            origin: metadata.origin,
            retrieved_at: metadata.retrieved_at,
            issuer: metadata.issuer,
            locator: metadata.locator,
            quality_notes: metadata.quality_notes,
            limitations: metadata.limitations,
            content_digest: object_id.clone(),
            object_id,
            byte_length: bytes.len() as u64,
        })
    }

    pub fn reconstruct(
        &self,
        manifest: &PacketManifest,
        expected_packet_id: &str,
        expected_version: u64,
    ) -> Result<PacketStudioDraft, StudioError> {
        if manifest.packet_id != expected_packet_id || manifest.packet_version != expected_version {
            return Err(StudioError::Integrity(
                "manifest packet identity/version mismatch".into(),
            ));
        }
        if manifest.manifest_digest != manifest.digest()? {
            return Err(StudioError::Integrity("manifest digest mismatch".into()));
        }
        let root_bytes = self
            .objects
            .get(&manifest.root_object_id)
            .ok_or_else(|| StudioError::MissingObject(manifest.root_object_id.clone()))?;
        if content_id_bytes(root_bytes) != manifest.root_object_id {
            return Err(StudioError::Integrity("root object digest mismatch".into()));
        }
        let draft: PacketStudioDraft =
            serde_json::from_slice(root_bytes).map_err(StudioError::json)?;
        if draft.packet_id != expected_packet_id || draft.version != expected_version {
            return Err(StudioError::Integrity(
                "root packet identity/version mismatch".into(),
            ));
        }
        let mut expected_entries = BTreeSet::new();
        for object in &draft.objects {
            let bytes = object.canonical_bytes()?;
            let object_id = content_id_bytes(&bytes);
            let entry = manifest
                .objects
                .iter()
                .find(|entry| entry.object_id == object_id);
            if entry.is_none_or(|entry| entry.kind != object.kind()) {
                return Err(StudioError::Integrity(format!(
                    "typed object {} is missing or has the wrong manifest kind",
                    object.id()
                )));
            }
            let stored = self
                .objects
                .get(&object_id)
                .ok_or_else(|| StudioError::MissingObject(object_id.clone()))?;
            if stored.as_slice() != bytes.as_slice() {
                return Err(StudioError::Integrity(format!(
                    "typed object {} was altered",
                    object.id()
                )));
            }
            expected_entries.insert((object_id, object.kind().to_owned()));
        }
        let actual_entries: BTreeSet<_> = manifest
            .objects
            .iter()
            .filter(|entry| entry.kind != "packet_draft")
            .map(|entry| (entry.object_id.clone(), entry.kind.clone()))
            .collect();
        if expected_entries != actual_entries {
            return Err(StudioError::Integrity(
                "manifest typed object set mismatch".into(),
            ));
        }
        Ok(draft)
    }
}

fn authorize_action(
    draft: &PacketStudioDraft,
    action: &str,
    grant: Option<&CapabilityGrant>,
    now: u64,
) -> Result<(), StudioError> {
    let state_digest = draft.state_digest()?;
    let request = AuthorizationRequest {
        principal: "local-editor".into(),
        action: action.into(),
        resource: format!("packet:{}", draft.packet_id),
        purpose: PUBLICATION_PURPOSE.into(),
        packet_id: draft.packet_id.clone(),
        state_digest,
    };
    if authorize(grant, &request, now) == Decision::Authorized {
        Ok(())
    } else {
        Err(StudioError::AuthorizationDenied(action.into()))
    }
}

fn parse_status(value: &str) -> Result<PacketStatus, StudioError> {
    match value {
        "draft" => Ok(PacketStatus::Draft),
        "source_frozen" => Ok(PacketStatus::SourceFrozen),
        "reviewed" => Ok(PacketStatus::Reviewed),
        "published" => Ok(PacketStatus::Published),
        other => Err(StudioError::InvalidPacket(format!(
            "unsupported status {other}"
        ))),
    }
}

fn required_string(value: &Value, field: &str) -> Result<String, StudioError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| StudioError::InvalidPacket(format!("missing string field {field}")))
}

fn optional_string(value: &Value, field: &str) -> Option<String> {
    value.get(field).and_then(Value::as_str).map(str::to_owned)
}

fn required_u64(value: &Value, field: &str) -> Result<u64, StudioError> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| StudioError::InvalidPacket(format!("missing integer field {field}")))
}

fn strings(value: &Value, field: &str) -> Result<Vec<String>, StudioError> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| StudioError::InvalidPacket(format!("missing array field {field}")))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| StudioError::InvalidPacket(format!("non-string item in {field}")))
        })
        .collect()
}

fn array(value: Option<&Value>) -> Result<&[Value], StudioError> {
    value
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| StudioError::InvalidPacket("missing object array".into()))
}
