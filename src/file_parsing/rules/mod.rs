use crate::{file_parsing::definitions::ConditionState, regex::NFA};

mod condition_state;
pub mod rules;
mod rules_states;

#[derive(Debug, Clone)]
pub struct RuleAction {
    pub nfa: NFA,
    pub action: String,
    pub condition_state: Vec<ConditionState>,
}

impl RuleAction {
    pub fn nfa(&self) -> &NFA {
        return &self.nfa;
    }

    pub fn action(&self) -> &String {
        return &self.action;
    }
    pub fn condition_state(&mut self) -> &mut Vec<ConditionState> {
        return &mut self.condition_state;
    }
}
