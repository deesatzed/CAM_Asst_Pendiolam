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
    Ok(format!("sha256:{:x}", Sha256::digest(canonical)))
}

pub fn canonical_json(packet_json: &str) -> Result<Vec<u8>, CanonicalizationError> {
    validate_packet(packet_json)
        .map_err(|error| CanonicalizationError(format!("invalid packet: {error}")))?;
    let value: Value = serde_json::from_str(packet_json)
        .map_err(|error| CanonicalizationError(format!("invalid JSON: {error}")))?;
    serde_json::to_vec(&canonicalize(value))
        .map_err(|error| CanonicalizationError(format!("cannot encode canonical JSON: {error}")))
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
