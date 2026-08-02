use common_reality_schema::validate_packet;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct CanonicalizationError(String);

impl fmt::Display for CanonicalizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for CanonicalizationError {}

pub fn content_id(packet_json: &str) -> Result<String, CanonicalizationError> {
    let canonical = canonical_json(packet_json)?;
    Ok(content_id_bytes(&canonical))
}

pub fn canonical_json(packet_json: &str) -> Result<Vec<u8>, CanonicalizationError> {
    validate_packet(packet_json)
        .map_err(|error| CanonicalizationError(format!("invalid packet: {error}")))?;
    let value: Value = serde_json::from_str(packet_json)
        .map_err(|error| CanonicalizationError(format!("invalid JSON: {error}")))?;
    Ok(canonical_json_value(&value))
}

/// Serializes any JSON value with recursively sorted object keys.
///
/// Gate 2 uses this helper for typed object and manifest identities that are
/// not themselves legacy Shared Reality Packets. The existing `content_id`
/// function remains the validating entry point for packet identities.
pub fn canonical_json_value(value: &Value) -> Vec<u8> {
    serde_json::to_vec(&canonicalize(value.clone()))
        .map_err(|error| CanonicalizationError(format!("cannot encode canonical JSON: {error}")))
        .expect("serde_json::Value is serializable")
}

/// Computes a stable content identity for any canonical JSON value.
pub fn content_id_value(value: &Value) -> String {
    content_id_bytes(&canonical_json_value(value))
}

/// Computes a SHA-256 content identity for already canonical bytes.
pub fn content_id_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn canonicalize(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(canonicalize).collect()),
        Value::Object(values) => {
            let sorted: BTreeMap<_, _> = values
                .into_iter()
                .map(|(key, value)| (key, canonicalize(value)))
                .collect();
            Value::Object(sorted.into_iter().collect())
        }
        scalar => scalar,
    }
}
