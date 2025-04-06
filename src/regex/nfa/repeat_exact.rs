use crate::NFA;

use super::{
    concatenate::concatenate,
    utils::{pop_last_two, shift_states},
};

pub fn repeat_exact(nfa: &NFA, count: usize) -> (NFA, usize) {
    let mut pieces = Vec::new();
    let mut offset = 0;

    if count == 0 {
        panic!("iteration value must be positive");
    }
    for _ in 0..count {
        let shifted = shift_states(nfa.clone(), offset);
        offset += shifted.transitions.len();
        pieces.push(shifted);

        if pieces.len() == 2 {
            let (left, right) = pop_last_two(&mut pieces);
            pieces.push(concatenate(left, right));
        }
    }

    let next_id = pieces.first().unwrap().final_states.iter().max().unwrap() + 1;
    return (pieces.pop().unwrap(), next_id);
}
