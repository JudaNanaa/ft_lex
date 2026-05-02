mod at_least;
pub mod automaton;
pub mod combine_nfa;
mod concatenate;
mod from_char;
mod offset;
mod or;
mod range;
mod repeat_exact;
mod utils;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transition {
    pub input: char,
    pub target_state: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nfa {
    pub transitions: HashMap<usize, Vec<Transition>>,
    pub final_states: HashSet<usize>,
    pub trailing_states: HashSet<usize>,
    pub trailing_final_states: HashSet<usize>,
}

impl Nfa {
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
            final_states: HashSet::new(),
            trailing_states: HashSet::new(),
            trailing_final_states: HashSet::new(),
        }
    }
}

impl Default for Nfa {
    fn default() -> Self {
        Self::new()
    }
}
