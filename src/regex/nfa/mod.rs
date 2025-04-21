mod at_least;
mod concatenate;
mod from_char;
pub mod nfa;
mod offset;
mod combine_nfa;
mod or;
mod range;
mod repeat_exact;
mod utils;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transition {
    pub input: char,
    pub target_state: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NFA {
    pub transitions: HashMap<usize, Vec<Transition>>,
    pub final_states: Vec<usize>,
}

impl NFA {
    pub fn new() -> Self {
        return Self {
            transitions: HashMap::new(),
            final_states: Vec::new(),
        };
    }
}
