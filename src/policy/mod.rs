use crate::gates::{evaluate_gate, GateDecision, GateResult, TopologyRiskAssessment};

pub fn authorize_action(
    existing_hard_block: bool,
    existing_soft_block: bool,
    assessment: &TopologyRiskAssessment,
) -> GateDecision {
    let topology_decision = evaluate_gate(assessment);

    if existing_hard_block || topology_decision.result == GateResult::HardBlock {
        return GateDecision {
            result: GateResult::HardBlock,
            reason: format!(
                "Policy hard block (existing={}, topology={})",
                existing_hard_block, topology_decision.reason
            ),
        };
    }

    if existing_soft_block || topology_decision.result == GateResult::SoftBlock {
        return GateDecision {
            result: GateResult::SoftBlock,
            reason: format!(
                "Policy soft block (existing={}, topology={})",
                existing_soft_block, topology_decision.reason
            ),
        };
    }

    GateDecision {
        result: GateResult::Allow,
        reason: format!("Policy allow ({})", topology_decision.reason),
    }
}

pub fn policy_gate(
    existing_hard_block: bool,
    existing_soft_block: bool,
    assessment: &TopologyRiskAssessment,
) -> GateDecision {
    authorize_action(existing_hard_block, existing_soft_block, assessment)
}

pub fn status() -> &'static str {
    "policy: ready"
}
