mod at_least;
pub mod combine_nfa;
mod concatenate;
mod from_char;
pub mod nfa;
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
pub struct NFA {
    pub transitions: HashMap<usize, Vec<Transition>>,
    pub final_states: HashSet<usize>,
    pub trailing_states: HashSet<usize>,
    pub trailing_final_states: HashSet<usize>,
}

impl NFA {
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
            final_states: HashSet::new(),
            trailing_states: HashSet::new(),
            trailing_final_states: HashSet::new(),
        }
    }

    pub fn compute_charset(&self) -> HashSet<char> {
        let mut charset = HashSet::new();
        for transitions in self.transitions.values() {
            for t in transitions {
                charset.insert(t.input);
            }
        }
        charset
    }
}

impl Default for NFA {
    fn default() -> Self {
        Self::new()
    }
}
