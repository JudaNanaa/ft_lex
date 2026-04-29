use crate::{file_parsing::definitions::ConditionState, regex::NFA};

mod condition_state;
pub mod rules;
mod rules_states;

#[derive(Debug, Clone)]
pub struct RuleAction {
    pub nfa: NFA,
    pub action: String,
    pub condition_state: Vec<ConditionState>,
    pub is_bol: bool,
    pub is_eol: bool,
}

impl RuleAction {
    pub fn nfa(&self) -> &NFA {
        &self.nfa
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn condition_state(&mut self) -> &mut Vec<ConditionState> {
        &mut self.condition_state
    }
}
