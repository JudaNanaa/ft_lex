use std::collections::{HashMap, HashSet};

pub mod automaton;
mod dot;
pub mod rule_actions;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    pub state: Vec<usize>,
}

impl State {
    pub fn trap() -> Self {
        Self { state: vec![0] }
    }

    pub fn is_trap(&self) -> bool {
        self.state == vec![0]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NewDfaTransition {
    pub input: char,
    pub target_state: usize,
}

impl NewDfaTransition {
    pub fn input(&self) -> &char {
        &self.input
    }

    pub fn target_state(&self) -> &usize {
        &self.target_state
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dfa {
    pub transitions: HashMap<usize, Vec<NewDfaTransition>>,
    pub final_states: HashSet<usize>,
    pub charset: HashSet<char>,
    pub nfa_states: HashMap<usize, Vec<usize>>,
    pub trailing_states: HashSet<usize>,
    pub trailing_final_states: HashSet<usize>,
}

impl Dfa {
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
            final_states: HashSet::new(),
            charset: HashSet::new(),
            nfa_states: HashMap::new(),
            trailing_states: HashSet::new(),
            trailing_final_states: HashSet::new(),
        }
    }

    pub fn charset(&self) -> &HashSet<char> {
        &self.charset
    }

    pub fn transitions(&self) -> &HashMap<usize, Vec<NewDfaTransition>> {
        &self.transitions
    }

    pub fn final_states(&self) -> &HashSet<usize> {
        &self.final_states
    }
}

impl Default for Dfa {
    fn default() -> Self {
        Self::new()
    }
}
