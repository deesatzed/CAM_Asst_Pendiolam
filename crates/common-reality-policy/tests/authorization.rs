use common_reality_policy::{authorize, AuthorizationRequest, CapabilityGrant, Decision};

fn request() -> AuthorizationRequest {
    AuthorizationRequest {
        principal: "participant-alex".into(),
        action: "submit_disagreement".into(),
        resource: "packet:campus-ai-v1".into(),
        purpose: "campus-deliberation".into(),
        packet_id: "campus-ai-v1".into(),
        state_digest: "sha256:current-state".into(),
    }
}

fn grant() -> CapabilityGrant {
    CapabilityGrant {
        issuer: "editor-1".into(),
        principal: "participant-alex".into(),
        action: "submit_disagreement".into(),
        resource: "packet:campus-ai-v1".into(),
        purpose: "campus-deliberation".into(),
        packet_id: "campus-ai-v1".into(),
        state_digest: "sha256:current-state".into(),
        expires_at: 200,
        revoked: false,
    }
}

#[test]
fn authorization_denies_by_default_and_permits_only_an_exact_live_grant() {
    let request = request();

    assert_eq!(authorize(None, &request, 100), Decision::Denied);
    assert_eq!(
        authorize(Some(&grant()), &request, 100),
        Decision::Authorized
    );
}

#[test]
fn authorization_rejects_scope_drift_expiry_and_revocation() {
    let request = request();
    let mut wrong_packet = grant();
    wrong_packet.packet_id = "campus-ai-v2".into();
    assert_eq!(
        authorize(Some(&wrong_packet), &request, 100),
        Decision::Denied
    );

    assert_eq!(authorize(Some(&grant()), &request, 200), Decision::Denied);

    let mut revoked = grant();
    revoked.revoked = true;
    assert_eq!(authorize(Some(&revoked), &request, 100), Decision::Denied);
}
