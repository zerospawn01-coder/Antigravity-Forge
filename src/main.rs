mod audit;
mod gates;
mod policy;
mod runtime;

use audit::{emit_event, AuditEvent, AuditEventKind};
use gates::{GateResult, TopologyRiskAssessment};
use policy::policy_gate;
use runtime::{execute_action, ExecutionRequest};

fn main() {
    let action = "deploy_module";

    // Phase E stub: topology assessment can later be replaced by JSON contract input.
    let assessment = TopologyRiskAssessment::stub(action);

    emit_event(&AuditEvent {
        kind: AuditEventKind::TopologyRiskAssessed,
        action: action.to_string(),
        detail: format!(
            "risk_score={} summary={}",
            assessment.risk_score, assessment.summary
        ),
    });

    let decision = policy_gate(false, false, &assessment);

    match decision.result {
        GateResult::HardBlock => emit_event(&AuditEvent {
            kind: AuditEventKind::GateTopologyBlocked,
            action: action.to_string(),
            detail: decision.reason.clone(),
        }),
        GateResult::SoftBlock => emit_event(&AuditEvent {
            kind: AuditEventKind::GateTopologySoftBlocked,
            action: action.to_string(),
            detail: decision.reason.clone(),
        }),
        GateResult::Allow => {}
    }

    let request = ExecutionRequest {
        action: action.to_string(),
        payload: None,
        gate_result: decision.result,
    };

    let execution_result = execute_action(&request);
    println!("Execution outcome: {:?}", execution_result.outcome);
    println!("Execution message: {}", execution_result.message);
}
