use std::collections::HashMap;

use super::{Transition, NFA};

pub fn from_char(c: char, state_id: &mut usize) -> NFA {
    let mut transitions: HashMap<usize, Vec<Transition>> = HashMap::new();
    let final_state = *state_id;

    let transition = Transition {
        input: c,
        target_state: final_state,
    };

    *state_id += 1;
    transitions.insert(0, vec![transition]);

    return NFA {
        transitions,
        final_states: vec![final_state],
    };
}
