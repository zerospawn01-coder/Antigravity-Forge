#[derive(Debug, Clone)]
pub enum AuditEventKind {
    TopologyRiskAssessed,
    GateTopologyBlocked,
    GateTopologySoftBlocked,
}

impl AuditEventKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TopologyRiskAssessed => "TOPOLOGY_RISK_ASSESSED",
            Self::GateTopologyBlocked => "GATE_TOPOLOGY_BLOCKED",
            Self::GateTopologySoftBlocked => "GATE_TOPOLOGY_SOFT_BLOCKED",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub kind: AuditEventKind,
    pub action: String,
    pub detail: String,
}

pub fn emit_event(event: &AuditEvent) {
    println!(
        "AUDIT kind={} action={} detail={}",
        event.kind.as_str(),
        event.action,
        event.detail
    );
}

pub fn status() -> &'static str {
    "audit: ready"
}
