use std::collections::HashSet;
use std::collections::VecDeque;

#[cfg(feature = "dotfile")]
use crate::regex::dfa::dot::generate_dot_file;

use super::{DfaTransition, State, DFA};
use crate::regex::dfa::NewDfaTransition;
use crate::regex::NFA;

fn new_final_states(dfa: &DFA) -> HashSet<usize> {
    let mut new_final_state = HashSet::new();

    for state in &dfa.final_states {
        if let Some(nb) = dfa.test.get(&state) {
            new_final_state.insert(*nb);
        }
    }

    return new_final_state;
}

fn final_states(dfa: &DFA, final_states: HashSet<usize>) -> HashSet<State> {
    let mut new_final_state = HashSet::new();

    for state in dfa.transitions.keys() {
        if state.state.iter().any(|s| final_states.contains(s)) {
            new_final_state.insert(state.clone());
        }
    }

    return new_final_state;
}

fn get_target_state_for_input(nfa: &NFA, current_state: &State, input_char: &char) -> State {
    let mut store: HashSet<usize> = HashSet::new();

    for nfa_state_id in &current_state.state {
        if let Some(nfa_transitions) = nfa.transitions.get(nfa_state_id) {
            for transition in nfa_transitions {
                if transition.input == *input_char {
                    store.insert(transition.target_state);
                }
            }
        }
    }

    if store.is_empty() {
        return State::trap();
    }
    let mut sorted_states: Vec<usize> = store.into_iter().collect();
    sorted_states.sort_unstable();
    return State {
        state: sorted_states,
    };
}

pub fn construct_dfa(nfa: NFA) -> DFA {
    let mut dfa = DFA::new();

    // All ASCII characters
    let alphabet = (0..=255u8)
        .filter_map(|c| char::from_u32(c as u32))
        .collect::<Vec<char>>();

    // Stack of DFA states to process
    let mut unprocessed_states = VecDeque::from(vec![State { state: vec![0] }]);

    dfa.test.insert(State { state: vec![0] }, 0);

    let mut new_state = 1;

    while let Some(current_state) = unprocessed_states.pop_front() {
        let mut transitions_from_current = Vec::with_capacity(alphabet.len());

        let mut new_transitions_from_current = Vec::with_capacity(alphabet.len());

        for input_char in &alphabet {
            let state = get_target_state_for_input(&nfa, &current_state, input_char);
            if !state.is_trap() {
                let nb = match dfa.test.contains_key(&state) {
                    true => *dfa.test.get(&state).unwrap(),
                    false => {
                        let n = new_state;
                        dfa.test.insert(state.clone(), new_state);
                        new_state += 1;
                        n
                    }
                };

                transitions_from_current.push(DfaTransition {
                    input: *input_char,
                    target_state: state,
                });

                new_transitions_from_current.push(NewDfaTransition {
                    input: *input_char,
                    target_state: nb,
                });
            }
        }
        dfa.new_transitions.insert(
            *dfa.test.get(&current_state).unwrap(),
            new_transitions_from_current.clone(),
        );

        dfa.transitions
            .insert(current_state, transitions_from_current.clone());

        for transition in transitions_from_current {
            if !dfa.transitions.contains_key(&transition.target_state)
                && !unprocessed_states.contains(&transition.target_state)
            {
                unprocessed_states.push_back(transition.target_state);
            }
        }
    }

    dfa.final_states = final_states(&dfa, nfa.final_states);
    dfa.new_final_states = new_final_states(&dfa);
    dfa.charset = nfa.charset;
    println!("nb state dfa == {}", dfa.transitions.len());

    dfa.new_final_states.remove(&0);
    #[cfg(feature = "dotfile")]
    match generate_dot_file(&dfa) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Unexpected error with dfa.dot generating {}", error);
        }
    }
    return dfa;
}
