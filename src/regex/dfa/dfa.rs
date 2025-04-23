use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use super::{DfaTransition, State, DFA};
use crate::regex::NFA;

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
    let alphabet = (0..=127u8)
        .filter_map(|c| char::from_u32(c as u32))
        .collect::<Vec<char>>();

    // Stack of DFA states to process
    let mut unprocessed_states = VecDeque::from(vec![State { state: vec![0] }]);

    while let Some(current_state) = unprocessed_states.pop_front() {
        let mut transitions_from_current = Vec::with_capacity(alphabet.len());

        for input_char in &alphabet {
            let state = get_target_state_for_input(&nfa, &current_state, input_char);
            if !state.is_trap() {
                transitions_from_current.push(DfaTransition {
                    input: *input_char,
                    target_state: state,
                });
            }
        }

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

    dfa.final_states = nfa.final_states;
    println!("nb state dfa == {}", dfa.transitions.len());
	generate_file_dot(&dfa);
    return dfa;
}

pub fn generate_file_dot(dfa: &DFA) -> std::io::Result<()> {
	let mut file = File::create("dfa.dot")?;

	file.write("digraph DFA {\n".as_bytes())?;
	file.write("  rankdir=LR;\n".as_bytes())?;
	file.write("  node [shape=circle];\n".as_bytes())?;

	// États finaux avec double cercle
	for state in dfa.transitions.keys() {
		if state.state.iter().any(|s| dfa.final_states.contains(s)) {
			writeln!(file, "  \"{:?}\" [shape=doublecircle];", state.state)?;
		}
	}

	for (from_state, transitions) in &dfa.transitions {
        for transition in transitions {
            // On ignore les pièges
            if transition.target_state.is_trap() {
                continue;
            }
            writeln!(
                file,
                "  \"{:?}\" -> \"{:?}\" [label=\"{}\"]",
                from_state.state,
                transition.target_state.state,
                transition.input
            )?;
        }
    }

	writeln!(file, "}}")?;


    Command::new("dot")
        .args(&["-Tpng", "dfa.dot", "-o", "dfa.png"])
        .output()
        .expect("Échec lors de l'exécution de Graphviz (dot)");
	return Ok(());
}