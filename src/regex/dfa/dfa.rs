use super::{DfaTransition, State, DFA};
use crate::regex::{Transition, NFA};

fn get_target_state_for_input(transitions: &Vec<Transition>, input_char: &char) -> State {
    let mut resulting_state = State { state: Vec::new() };

    for transition in transitions {
        if transition.input == *input_char {
            resulting_state.state.push(transition.target_state);
        }
    }

    if resulting_state.state.is_empty() {
        return State::trap(); // dead-end state
    }

    resulting_state.state.sort();
    return resulting_state;
}

pub fn construct_dfa(nfa: NFA) -> DFA {
    let mut dfa = DFA::new();

    // All ASCII characters
    let alphabet = (0..=127u8)
        .filter_map(|c| char::from_u32(c as u32))
        .collect::<Vec<char>>();

    // Stack of DFA states to process
    let mut unprocessed_states = vec![State { state: vec![0] }];

    while let Some(current_state) = unprocessed_states.pop() {
        let mut transitions_from_current = Vec::new();

        for nfa_state_id in &current_state.state {
            let nfa_transitions = match nfa.transitions.get(nfa_state_id) {
                Some(transitions) => transitions,
                None => continue,
            };

            for input_char in &alphabet {
                let target_state = get_target_state_for_input(nfa_transitions, input_char);
                transitions_from_current.push(DfaTransition {
                    input: *input_char,
                    target_state,
                });
            }
        }

        dfa.transitions
            .insert(current_state, transitions_from_current.clone());

        for transition in transitions_from_current {
            if !dfa.transitions.contains_key(&transition.target_state)
                && !unprocessed_states.contains(&transition.target_state)
            {
                unprocessed_states.push(transition.target_state);
            }
        }
    }

    dfa.final_states = nfa.final_states;
    return dfa;
}
