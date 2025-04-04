use crate::regex::utils::RemoveVecElement;
use crate::regex::Operator;
use crate::regex::Quantifier;
use crate::regex::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(usize);

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Transition {
    input: char,
    target_state: usize,
}

#[derive(Debug, Clone)]
pub struct Automaton {
    transitions: HashMap<usize, Vec<Transition>>,
    final_states: Vec<usize>,
}

fn pop_last_two_automata(stack: &mut Vec<Automaton>) -> (Automaton, Automaton) {
    let second = stack.pop().expect("Internal error");
    let first = stack.pop().expect("Internal error");
    
	return (first, second);
}

fn create_automaton_from_char(input: char, state_counter: &mut usize) -> Automaton {
    let mut transition_map: HashMap<usize, Vec<Transition>> = HashMap::new();

    let current_state = *state_counter;
    let transition = Transition {
        input,
        target_state: *state_counter,
    };
    
    *state_counter += 1;
    
    transition_map.insert(0, vec![transition]);

    return Automaton {
        transitions: transition_map,
        final_states: vec![current_state],
    };
}

fn apply_kleene_star(automaton: &mut Automaton) {
    let final_states = &automaton.final_states;
    let transitions = &mut automaton.transitions;

    let initial_transitions = transitions.get(&0).cloned().expect("No initial state, internal error");

    for state in final_states {
		transitions.entry(*state)
			.or_insert_with(Vec::new)
			.extend(initial_transitions.clone());
    }
    automaton.final_states.push(0);
}

fn concatenate_automata(mut left: Automaton, mut right: Automaton) -> Automaton {
    let left_final_states = &left.final_states;
    let left_transitions = &mut left.transitions;
    let right_initial_transitions = right.transitions.remove(&0).unwrap_or_default();
    let right_has_final_initial_state = right.final_states.contains(&0);

	right.final_states.remove_element(&0);
    for &state in left_final_states {
        left_transitions.entry(state)
            .or_insert_with(Vec::new)
            .extend(right_initial_transitions.clone());

        if right_has_final_initial_state {
            right.final_states.push(state);
        }
    }

    left_transitions.extend(right.transitions);

    return Automaton {
        transitions: left.transitions,
        final_states: right.final_states,
    };
}

fn or_automata(left: Automaton, right: Automaton) -> Automaton {
	let mut final_states = left.final_states;
	let mut transitions = left.transitions;

	for mut trans in right.transitions {
		match transitions.contains_key(&trans.0) {
			false => {
				transitions.insert(trans.0, trans.1);
			},
			true => {
				let array = transitions.get_mut(&trans.0).unwrap();
				array.append(&mut trans.1);
			}
		}
	}
	for state in right.final_states {
		if final_states.contains(&state) == false {
			final_states.push(state);
		}
	}
	
	return Automaton {
		transitions,
		final_states,
	};
}

pub fn construct_nfa(tokens: &Vec<Token>) -> Automaton {
    let mut automaton_stack: Vec<Automaton> = Vec::new();
    let mut state_counter = 1;

    for token in tokens {
        let automaton = match *token {
            Token::Char(input) => create_automaton_from_char(input, &mut state_counter),
            Token::Operator(operator) => match operator {
                Operator::Quantifier(quantifier) => match quantifier {
                    Quantifier::AtLeast(nb) if nb == 0 => {
                        let mut automaton = automaton_stack.pop().expect("Error applying Kleene star");
                        apply_kleene_star(&mut automaton);
                        automaton
                    },
                    _ => todo!(),
                }
                Operator::Concatenation => {
                    let (first, second) = pop_last_two_automata(&mut automaton_stack);
                    concatenate_automata(first, second)
                },
				Operator::Or => {
					let (first, second) = pop_last_two_automata(&mut automaton_stack);
					or_automata(first, second)
				}
                _ => todo!(),
            },
        };
        dbg!(&automaton);
        automaton_stack.push(automaton);
    }

    todo!();
}
