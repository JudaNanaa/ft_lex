use std::collections::HashMap;

pub mod dfa;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
	state: Vec<usize>,
	is_final: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DfaTransition {
	input: char,
	target_state: State,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DFA {
	transitions: HashMap<State, Vec<DfaTransition>>,
}

impl DFA {
    pub fn new() -> Self {
        return Self {
            transitions: HashMap::new(),
        };
    }
}