use common_reality_schema::validate_packet;

const PACKET: &str = include_str!("../../../fixtures/campus-ai-v1/packet.json");
const SCHEMA: &str = include_str!("../../../schemas/shared-reality-packet-v1.schema.json");

#[test]
fn accepts_the_frozen_campus_packet() {
    validate_packet(PACKET).expect("the frozen packet must satisfy the strict contract");
}

#[test]
fn rejects_an_unknown_nested_claim_field() {
    let invalid_packet = PACKET.replace(
        "\"claim_type\": \"association\",",
        "\"claim_type\": \"association\", \"secret_persuasion_score\": 1,",
    );

    assert!(validate_packet(&invalid_packet).is_err());
}

#[test]
fn rejects_a_material_claim_without_counterevidence_disclosure() {
    let invalid_packet = PACKET.replace(
        "\"evidence_against\": [\"evidence-accessibility\"],",
        "\"evidence_against\": [],",
    );

    assert!(validate_packet(&invalid_packet).is_err());
}

#[test]
fn rejects_a_forbidden_lens_profile_field() {
    let invalid_packet =
        PACKET.replace("\"language\",", "\"language\", \"political_affiliation\",");

    assert!(validate_packet(&invalid_packet).is_err());
}

#[test]
fn publishes_a_closed_json_schema_for_packet_and_claim_objects() {
    let schema: serde_json::Value =
        serde_json::from_str(SCHEMA).expect("schema must be valid JSON");

    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(schema["$defs"]["claim"]["additionalProperties"], false);
    assert_eq!(schema["$defs"]["evidence"]["additionalProperties"], false);
}
