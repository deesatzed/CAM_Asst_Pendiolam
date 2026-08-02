use common_reality_canonical::{canonical_json_value, content_id_value};
use serde_json::json;

#[test]
fn generic_values_have_deterministic_canonical_bytes_and_content_ids() {
    let first = json!({"b": [2, {"z": true, "a": "x"}], "a": 1});
    let reordered = json!({"a": 1, "b": [2, {"a": "x", "z": true}]});

    assert_eq!(
        canonical_json_value(&first),
        canonical_json_value(&reordered)
    );
    assert_eq!(content_id_value(&first), content_id_value(&reordered));
    assert_ne!(content_id_value(&first), content_id_value(&json!({"a": 2})));
}
