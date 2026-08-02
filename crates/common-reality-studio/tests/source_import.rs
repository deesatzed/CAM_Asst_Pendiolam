use common_reality_studio::{ObjectStore, SourceMetadata, StudioError};
use std::path::Path;

#[test]
fn bounded_file_import_keeps_provenance_and_stable_content_id() {
    let mut first_store = ObjectStore::new();
    let mut second_store = ObjectStore::new();
    let metadata = SourceMetadata::synthetic(
        "source-corpus",
        "Synthetic source corpus",
        "fixture",
        "2026-08-01T00:00:00Z",
    );
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/campus-ai-v1/source-corpus.json");

    let first = first_store
        .import_source_file(&path, 16_384, metadata.clone())
        .expect("bounded fixture import");
    let second = second_store
        .import_source_file(&path, 16_384, metadata)
        .expect("same fixture import");

    assert_eq!(first.content_digest, second.content_digest);
    assert_eq!(first.object_id, first.content_digest);
    assert_eq!(first.byte_length, second.byte_length);
    assert_eq!(first.origin, "fixture");
    assert!(first_store.contains(&first.object_id));
    assert!(!serde_json::to_string(&first).unwrap().contains("sources"));
}

#[test]
fn source_import_rejects_files_over_the_explicit_bound() {
    let mut store = ObjectStore::new();
    let metadata = SourceMetadata::synthetic(
        "source-corpus",
        "Synthetic source corpus",
        "fixture",
        "2026-08-01T00:00:00Z",
    );

    let error = store
        .import_source_file(
            &Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../fixtures/campus-ai-v1/source-corpus.json"),
            4,
            metadata,
        )
        .expect_err("the fixture must exceed the four-byte bound");

    assert!(matches!(error, StudioError::SourceTooLarge { .. }));
}
