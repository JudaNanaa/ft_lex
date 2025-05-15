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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NewDfaTransition {
    input: char,
    target_state: usize,
}

impl NewDfaTransition {
    pub fn input(&self) -> &char {
        return &self.input;
    }

    pub fn target_state(&self) -> &usize {
        return &self.target_state;
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct DFA {
    transitions: HashMap<State, Vec<DfaTransition>>,
    final_states: HashSet<State>,
    test: HashMap<State, usize>,
    new_transitions: HashMap<usize, Vec<NewDfaTransition>>,
    new_final_states: HashSet<usize>,
    charset: HashSet<char>,
}

impl DFA {
    pub fn new() -> Self {
        return Self {
            transitions: HashMap::new(),
            final_states: HashSet::new(),
            test: HashMap::new(),
            new_transitions: HashMap::new(),
            new_final_states: HashSet::new(),
            charset: HashSet::new(),
        };
    }

    pub fn charset(&self) -> &HashSet<char> {
        return &self.charset;
    }

    pub fn new_transitions(&self) -> &HashMap<usize, Vec<NewDfaTransition>> {
        return &self.new_transitions;
    }

    pub fn new_final_states(&self) -> &HashSet<usize> {
        return &self.new_final_states;
    }
}

use std::fmt::{Formatter, Result as FmtResult};

impl Debug for DFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("DFA")
            .field("new_transitions", &self.new_transitions)
            .field("new_final_states", &self.new_final_states)
            .field("charset", &self.charset)
            .finish()
    }
}
