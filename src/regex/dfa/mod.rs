use std::collections::HashSet;
use std::{collections::HashMap, fmt::Debug};

pub mod dfa;
mod dot;
pub mod rule_actions;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    state: Vec<usize>,
}

impl State {
    pub fn trap() -> Self {
        return Self { state: vec![0] };
    }

    pub fn is_trap(&self) -> bool {
        return self.state == vec![0];
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DfaTransition {
    input: char,
    target_state: State,
}

#[derive(Clone, PartialEq, Eq)]
pub struct DFA {
    transitions: HashMap<State, Vec<DfaTransition>>,
    final_states: HashSet<State>,
}

impl DFA {
    pub fn new() -> Self {
        return Self {
            transitions: HashMap::new(),
            final_states: HashSet::new(),
        };
    }
}

use std::fmt::{Formatter, Result as FmtResult};

impl Debug for DFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let filtered: HashMap<_, Vec<_>> = self
            .transitions
            .iter()
            .map(|(state, transitions)| {
                let filtered_transitions: Vec<_> = transitions
                    .iter()
                    .filter(|t| !t.target_state.is_trap())
                    .cloned()
                    .collect();
                (state, filtered_transitions)
            })
            .filter(|(_, transitions)| !transitions.is_empty())
            .collect();

        f.debug_struct("DFA")
            .field("transitions", &filtered)
            .field("final_states", &self.final_states)
            .finish()
    }
}
