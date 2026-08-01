use common_reality_canonical::content_id;

const PACKET: &str = include_str!("../../../fixtures/campus-ai-v1/packet.json");

#[test]
fn equivalent_packet_key_order_has_the_same_content_id() {
    let original = content_id(PACKET).expect("frozen packet must canonicalize");
    let reordered = reorder_object_keys(PACKET);

    assert_eq!(
        original,
        content_id(&reordered).expect("reordered packet must canonicalize")
    );
}

#[test]
fn one_bit_semantic_change_has_a_new_content_id() {
    let original = content_id(PACKET).expect("frozen packet must canonicalize");
    let changed = PACKET.replacen("\"moderate\"", "\"low\"", 1);

    assert_ne!(
        original,
        content_id(&changed).expect("changed packet must canonicalize")
    );
}

fn reorder_object_keys(json: &str) -> String {
    fn reorder(value: serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Array(values) => {
                serde_json::Value::Array(values.into_iter().map(reorder).collect())
            }
            serde_json::Value::Object(values) => {
                let mut entries: Vec<_> = values.into_iter().collect();
                entries.reverse();
                serde_json::Value::Object(
                    entries
                        .into_iter()
                        .map(|(key, value)| (key, reorder(value)))
                        .collect(),
                )
            }
            scalar => scalar,
        }
    }

    serde_json::to_string(&reorder(serde_json::from_str(json).expect("fixture JSON")))
        .expect("JSON serialization")
}
