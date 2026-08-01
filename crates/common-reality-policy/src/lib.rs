#[derive(Debug, PartialEq, Eq)]
pub enum Decision {
    Authorized,
    Denied,
}

pub struct AuthorizationRequest {
    pub principal: String,
    pub action: String,
    pub resource: String,
    pub purpose: String,
    pub packet_id: String,
    pub state_digest: String,
}

pub struct CapabilityGrant {
    pub issuer: String,
    pub principal: String,
    pub action: String,
    pub resource: String,
    pub purpose: String,
    pub packet_id: String,
    pub state_digest: String,
    pub expires_at: u64,
    pub revoked: bool,
}

pub fn authorize(
    grant: Option<&CapabilityGrant>,
    request: &AuthorizationRequest,
    now: u64,
) -> Decision {
    let Some(grant) = grant else {
        return Decision::Denied;
    };

    if grant.revoked || now >= grant.expires_at || grant.issuer.is_empty() {
        return Decision::Denied;
    }

    if grant.principal == request.principal
        && grant.action == request.action
        && grant.resource == request.resource
        && grant.purpose == request.purpose
        && grant.packet_id == request.packet_id
        && grant.state_digest == request.state_digest
    {
        Decision::Authorized
    } else {
        Decision::Denied
    }
}
