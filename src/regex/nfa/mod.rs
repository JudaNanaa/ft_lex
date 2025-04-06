mod at_least;
mod concatenate;
mod from_char;
pub mod nfa;
mod or;
mod range;
mod repeat_exact;
mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(usize);

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transition {
    input: char,
    target_state: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NFA {
    transitions: HashMap<usize, Vec<Transition>>,
    final_states: Vec<usize>,
}

impl NFA {
    pub fn new() -> Self {
        return Self {
            transitions: HashMap::new(),
            final_states: Vec::new(),
        };
    }
}
