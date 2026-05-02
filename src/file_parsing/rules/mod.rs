use std::collections::HashSet;

use crate::{file_parsing::definitions::ConditionState, regex::Nfa};

mod condition_state;
pub mod parse;
mod rules_states;

#[derive(Debug, Clone)]
pub struct RuleAction {
    pub nfa: Nfa,
    pub action: String,
    pub condition_state: Vec<ConditionState>,
    pub anchored_start: bool,
    pub anchored_end: bool,
    pub charsets: Vec<HashSet<char>>,
}

impl RuleAction {
    pub fn nfa(&self) -> &Nfa {
        &self.nfa
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn condition_state(&mut self) -> &mut Vec<ConditionState> {
        &mut self.condition_state
    }
}
