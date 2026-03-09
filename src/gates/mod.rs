#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateResult {
    Allow,
    SoftBlock,
    HardBlock,
}

#[derive(Debug, Clone)]
pub struct GateDecision {
    pub result: GateResult,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct TopologyRiskAssessment {
    pub risk_score: u8,
    pub hard_block: bool,
    pub soft_block: bool,
    pub summary: String,
}

impl TopologyRiskAssessment {
    pub fn stub(action: &str) -> Self {
        if action.contains("danger") {
            return Self {
                risk_score: 95,
                hard_block: true,
                soft_block: false,
                summary: "High-risk topology detected".to_string(),
            };
        }

        Self {
            risk_score: 25,
            hard_block: false,
            soft_block: false,
            summary: "Topology appears stable".to_string(),
        }
    }
}

pub fn evaluate_gate(assessment: &TopologyRiskAssessment) -> GateDecision {
    if assessment.hard_block {
        return GateDecision {
            result: GateResult::HardBlock,
            reason: format!("Topology hard block: {}", assessment.summary),
        };
    }

    if assessment.soft_block {
        return GateDecision {
            result: GateResult::SoftBlock,
            reason: format!("Topology soft block: {}", assessment.summary),
        };
    }

    GateDecision {
        result: GateResult::Allow,
        reason: format!("Topology allow: {}", assessment.summary),
    }
}

pub fn status() -> &'static str {
    "gates: ready"
}
