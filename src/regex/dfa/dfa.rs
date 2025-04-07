use crate::regex::{Transition, NFA};

use super::{DfaTransition, State, DFA};

fn get_transitions_for_input(transition: &Vec<Transition>, char: &char) -> State {
	let mut output = State { state: Vec::new() };

	for t in transition {
		if t.input == *char {
			output.state.push(t.target_state);
		}
	}
	
	if output.state.is_empty() {
		return State::trap();
	}
	output.state.sort();
	return output;
}

fn set_state(state: State, nfa: &NFA, dfa: &mut DFA, all_chars: &Vec<char>) {
	let mut vec_of_transitions = Vec::new();


	for id in &state.state {
		let initial_state = match nfa.transitions.get(id) {
			Some(elem) => elem,
			None => continue,
		};
		for char in all_chars {
			let state = get_transitions_for_input(initial_state, char);
			vec_of_transitions.push(DfaTransition {
				input: *char,
				target_state: state,
			});
		}
	}
	dfa.transitions.insert(state,  vec_of_transitions.clone());

	for transition in vec_of_transitions {
		if !dfa.transitions.contains_key(&transition.target_state) {
			set_state(transition.target_state, nfa, dfa, all_chars);
		}
	}
}

pub fn construct_dfa(nfa: NFA) -> DFA {
	let mut new_dfa = DFA::new();

	let all_chars = (0..=127u8)
        .filter_map(|c| char::from_u32(c as u32))
        .collect::<Vec<char>>();

	set_state(State { state: vec![0] }, &nfa, &mut new_dfa, &all_chars);
	new_dfa.final_states = nfa.final_states;
	println!("nb of transitions == {}", new_dfa.transitions.len());
	return new_dfa;
}