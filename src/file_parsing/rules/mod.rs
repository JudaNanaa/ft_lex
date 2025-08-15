use crate::regex::NFA;

pub mod rules;
mod rules_states;

#[derive(Debug, Clone)]
pub struct RuleAction {
    nfa: NFA,
    action: String,
}

impl RuleAction {
    pub fn nfa(&self) -> &NFA {
        return &self.nfa;
    }

    pub fn action(&self) -> &String {
        return &self.action;
    }

    pub fn action_mut(&mut self) -> &mut String {
        return &mut self.action;
    }
}
