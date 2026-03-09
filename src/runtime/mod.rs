use crate::gates::GateResult;

#[derive(Debug, Clone)]
pub struct ExecutionRequest {
    pub action: String,
    pub payload: Option<String>,
    pub gate_result: GateResult,
}

#[derive(Debug, Clone)]
pub enum ExecutionOutcome {
    Executed,
    SoftBlockedExecuted,
    NoExecuteOverride,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub outcome: ExecutionOutcome,
    pub message: String,
}

pub fn execute_action(request: &ExecutionRequest) -> ExecutionResult {
    match request.gate_result {
        GateResult::HardBlock => ExecutionResult {
            outcome: ExecutionOutcome::NoExecuteOverride,
            message: format!(
                "NO_EXECUTE_OVERRIDE: action '{}' was blocked before execution",
                request.action
            ),
        },
        GateResult::SoftBlock => ExecutionResult {
            outcome: ExecutionOutcome::SoftBlockedExecuted,
            message: format!(
                "SOFT_BLOCK: action '{}' executed under constrained mode",
                request.action
            ),
        },
        GateResult::Allow => ExecutionResult {
            outcome: ExecutionOutcome::Executed,
            message: format!("ALLOW: action '{}' executed", request.action),
        },
    }
}

pub fn status() -> &'static str {
    "runtime: ready"
}
