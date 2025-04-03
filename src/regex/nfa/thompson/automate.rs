// use std::collections::HashSet;

use crate::regex::Operator;
use crate::regex::Quantifier;
use crate::regex::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(usize);

// #[derive(Debug, Clone)]
// pub struct Transition {
//     from: State,
//     to: State,
//     input: Option<char>,
// }

// #[derive(Debug)]
// pub struct Automaton {
//     transitions: Vec<Transition>,
//     initial_state: State,
//     final_states: HashSet<State>,
// }

// fn create_automata_char(char: char) -> Automaton {
//     let q0 = State(0);
//     let q1 = State(1);

//     let transition = Transition {
//         from: q0,
//         to: q1,
//         input: Some(char),
//     };

//     let mut final_s = HashSet::new();
//     final_s.insert(q1);

//     return Automaton {
//         transitions: vec![transition],
//         initial_state: q0,
//         final_states: final_s,
//     };
// }

// fn concatenation_two_automata(left: Automaton, right: Automaton) -> Automaton {

//     todo!();
// }

// fn pop_two_last_element_stack(stack: &mut Vec<Automaton>) -> (Automaton, Automaton) {
//     let right = match stack.pop() {
//         Some(elem) => elem,
//         None => panic!("Internal error"),
//     };
//     let left = match stack.pop() {
//         Some(elem) => elem,
//         None => panic!("Internal error"),
//     };
//     return (left, right);
// }

// // a
// pub fn create_nfa(tokens: &Vec<Token>) -> Automaton {
//     let mut stack: Vec<Automaton> = Vec::new();

//     for token in tokens {
//         let automaton = match *token {
//             Token::Char(c) => create_automata_char(c),
//             Token::Operator(op) => match op {
//                 Operator::Concatenation => {
//                     let (left, right) = pop_two_last_element_stack(&mut stack);
//                     concatenation_two_automata(left, right)
//                 }
//                 _ => todo!(),
//             },
//         };
//         println!("automaton = {:#?}", automaton);
//         stack.push(automaton);
//     }
//     dbg!(stack);
//     // if | -> link les deux last de la stack en mode or
//     // if concat -> concatener les deux last de la stack
//     // if quantifier -> faire le quantifier sur le dernier de la stack

//     // return stack.pop().unwrap();
// }

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Transition {
    input: char,
    to: usize,
}

#[derive(Debug, Clone)]
pub struct Automaton {
    transition_table: HashMap<usize, Vec<Transition>>,
	final_states: Vec<usize>,
}

fn pop_two_last_element_stack(stack: &mut Vec<Automaton>) -> (Automaton, Automaton) {
    let right = match stack.pop() {
        Some(elem) => elem,
        None => panic!("Internal error"),
    };
    let left = match stack.pop() {
        Some(elem) => elem,
        None => panic!("Internal error"),
    };
    return (left, right);
}

fn create_automata_char(char: char, state_number: &mut usize) -> Automaton {
    let mut table_of_transitions: HashMap<usize, Vec<Transition>> = HashMap::new();

	let state = *state_number;
    let transition = Transition {
        input: char,
        to: *state_number,
    };
	
	*state_number += 1;
	
    table_of_transitions.insert(0, vec![transition]);

    return Automaton {
        transition_table: table_of_transitions,
		final_states: vec![state],
    };
}

fn kleene_star(automaton: &mut Automaton) {
	let final_states = &automaton.final_states;
	let transition_table = &mut automaton.transition_table;

	let first_transition = match transition_table.get(&0) {
		Some(tab) => tab.clone(),
		None => panic!("No initial state, internal error"),
	};

	for state in final_states {

		match transition_table.get_mut(state) {
			Some(tab) => {
				*tab = first_transition.clone();
			}
			None => {
				transition_table.insert(*state, first_transition.clone());
			},	
		}
	}
	// automaton.final_states.push(0);
}

fn concatenation(mut left: Automaton, mut right: Automaton) -> Automaton {
	let mut new = left.clone();
	let left_final_states = &left.final_states;
	let left_transition_table = &mut left.transition_table;

	let right_initial_state = right.transition_table.get(&0).unwrap();
	
	for state in left_final_states {
		match left_transition_table.get_mut(state) {
			Some(tab) => {
				*tab = right_initial_state.clone();
			}
			None => {
				left_transition_table.insert(*state, right_initial_state.clone());
			},
		}	
	}
	right.transition_table.remove(&0);
	left_transition_table.extend(right.transition_table);

	return Automaton {
		transition_table: left.transition_table,
		final_states: right.final_states,
	};
}

pub fn create_nfa(tokens: &Vec<Token>) -> Automaton {
    let mut automaton_stack: Vec<Automaton> = Vec::new();
    let mut state_number = 1;

    for token in tokens {
        let automaton = match *token {
            Token::Char(char) => create_automata_char(char, &mut state_number),
            Token::Operator(operator) => match operator {
				Operator::Quantifier(quantifier) => match quantifier {
					Quantifier::AtLeast(nb) if nb == 0 => {
						let mut automaton = match automaton_stack.pop() {
							Some(elem) => elem,
							None => panic!("ERRor kleene star"),
						};
						kleene_star(&mut automaton);
						automaton
					},
					_ => todo!(),
				}
				Operator::Concatenation => {
					let (left, right) = pop_two_last_element_stack(&mut automaton_stack);
					concatenation(left, right)
				},
                _ => todo!(),
            },
        };
        dbg!(&automaton);
        automaton_stack.push(automaton);
    }

    todo!();
}
