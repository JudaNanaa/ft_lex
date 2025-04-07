use std::{collections::HashMap, fmt::Debug};

pub mod dfa;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
	state: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DfaTransition {
	input: char,
	target_state: State,
}

#[derive(Clone, PartialEq, Eq)]
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

use std::fmt::{Formatter, Result as FmtResult};

impl Debug for DFA {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let filtered: HashMap<_, Vec<_>> = self.transitions
			.iter()
			.map(|(state, transitions)| {
				let filtered_transitions: Vec<_> = transitions
					.iter()
					.filter(|t| t.target_state.state != vec![0])
					.cloned()
					.collect();
				(state, filtered_transitions)
			})
			.filter(|(_, transitions)| !transitions.is_empty())
			.collect();

		f.debug_struct("DFA")
			.field("transitions", &filtered)
			.finish()
	}
}
