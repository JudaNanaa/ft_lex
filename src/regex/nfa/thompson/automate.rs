use crate::regex::utils::RemoveVecElement;
use crate::regex::{Operator, Quantifier, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(usize);

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Transition {
    input: char,
    target_state: usize,
}

#[derive(Debug, Clone)]
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

fn pop_last_two(stack: &mut Vec<NFA>) -> (NFA, NFA) {
    let second = stack.pop().expect("Internal error");
    let first = stack.pop().expect("Internal error");
    return (first, second);
}

fn deduplicate_transitions(transitions: &mut HashMap<usize, Vec<Transition>>) {
    for (_state, list) in transitions.iter_mut() {
        let mut set: HashSet<Transition> = HashSet::new();
        set.extend(list.drain(..)); // vide la liste dans le set
        list.extend(set.into_iter()); // remet les éléments uniques
    }
}


fn from_char(c: char, state_id: &mut usize) -> NFA {
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

fn apply_kleene_star(nfa: &mut NFA) {
    let initial_transitions = nfa.transitions.get(&0)
        .cloned()
        .expect("No initial state, internal error");

    for &final_state in &nfa.final_states {
        let entry = nfa.transitions.entry(final_state).or_insert_with(Vec::new);
        let mut unique: HashSet<_> = entry.iter().cloned().collect();
        unique.extend(initial_transitions.clone());
        *entry = unique.into_iter().collect();
    }

    if !nfa.final_states.contains(&0) {
        nfa.final_states.push(0);
    }
}

fn concatenate(mut left: NFA, mut right: NFA) -> NFA {
    let right_initial = right.transitions.remove(&0).unwrap_or_default();
    let right_has_initial_final = right.final_states.contains(&0);

    right.final_states.remove_element(&0);

    for &state in &left.final_states {
        left.transitions.entry(state)
            .or_insert_with(Vec::new)
            .extend(right_initial.clone());

        if right_has_initial_final {
            right.final_states.push(state);
        }
    }

    left.transitions.extend(right.transitions);

    return NFA {
        transitions: left.transitions,
        final_states: right.final_states,
    };
}

fn alternate(left: NFA, right: NFA) -> NFA {
    let mut transitions = left.transitions;
    let mut final_states = left.final_states;

    for (state, mut trans) in right.transitions {
        transitions.entry(state)
            .and_modify(|list| list.append(&mut trans))
            .or_insert(trans);
    }

    for state in right.final_states {
        if !final_states.contains(&state) {
            final_states.push(state);
        }
    }

    return NFA {
        transitions,
        final_states,
    };
}

fn shift_states(nfa: NFA, offset: usize) -> NFA {
    let mut new_nfa = NFA::new();

    for (state, transitions) in nfa.transitions {
        let new_key = if state == 0 { 0 } else { state + offset };
        let updated_transitions: Vec<Transition> = transitions
            .into_iter()
            .map(|mut t| {
                t.target_state += offset;
                t
            })
            .collect();
        new_nfa.transitions.insert(new_key, updated_transitions);
    }

    for final_state in nfa.final_states {
        let new_final = if final_state == 0 { 0 } else { final_state + offset };
        new_nfa.final_states.push(new_final);
    }

    return new_nfa;
}

fn repeat_exact(nfa: &NFA, count: usize) -> (NFA, usize) {
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

fn at_least(nfa: NFA, count: usize) -> (NFA, usize) {
    if count == 0 {
        let mut kleene = nfa.clone();
        apply_kleene_star(&mut kleene);
		let next_id = kleene.final_states.iter().max().unwrap() + 1;
        return (kleene, next_id);
    }

    let (repeated, _) = repeat_exact(&nfa, count);
    let mut kleene_part = nfa.clone();
    apply_kleene_star(&mut kleene_part);

    let shifted_kleene = shift_states(kleene_part, repeated.transitions.len());
    let result = concatenate(repeated, shifted_kleene);
	let next_id = result.final_states.iter().max().unwrap() + 1;
    return (result, next_id);
}

fn range(nfa: NFA, min: usize, max: usize) -> (NFA, usize) {
    assert!(min <= max, "Invalid range");

    if min == max {
        return repeat_exact(&nfa, min);
    }

    let mut nfa_parts: Vec<NFA> = Vec::new();
    let mut total_offset = 0;
    let mut accumulated_final_states = Vec::new();

    // Partie obligatoire : min répétitions
    if min > 0 {
        let (mandatory_nfa, _) = repeat_exact(&nfa, min);
        accumulated_final_states = mandatory_nfa.final_states.clone();
        total_offset += mandatory_nfa.transitions.len();
        nfa_parts.push(mandatory_nfa);
    } else {
        accumulated_final_states.push(0); // L’état initial est final si min == 0
    }

    // Parties optionnelles : (max - min) répétitions
    for _ in min..max {
        let optional_nfa = shift_states(nfa.clone(), total_offset);

        if let Some(transitions_from_initial) = optional_nfa.transitions.get(&0) {
            for transition in transitions_from_initial {
                accumulated_final_states.push_unique(transition.target_state);
            }
        }

        total_offset += optional_nfa.transitions.len();
        nfa_parts.push(optional_nfa);

        if nfa_parts.len() == 2 {
            let (left, right) = pop_last_two(&mut nfa_parts);
            nfa_parts.push(concatenate(left, right));
        }
    }

    let mut final_nfa = nfa_parts.pop().unwrap();
    for state in accumulated_final_states {
        final_nfa.final_states.push_unique(state);
    }

    return (final_nfa, total_offset + 1);
}


pub fn construct_nfa(tokens: &Vec<Token>) -> NFA {
    let mut stack: Vec<NFA> = Vec::new();
    let mut state_id = 1;

    for token in tokens {
        let nfa = match *token {
            Token::Char(c) => from_char(c, &mut state_id),
            Token::Operator(op) => match op {
                Operator::Quantifier(q) => match q {
                    Quantifier::AtLeast(n) => {
                        let base = stack.pop().expect("Error applying Kleene star");
						let (new_nfa, new_id) = at_least(base, n);
						state_id = new_id;
						new_nfa
                    }
                    Quantifier::Equal(n) => {
                        let base = stack.pop().expect("Error applying Equal");
                        let (new_nfa, new_id) = repeat_exact(&base, n);
                        state_id = new_id;
                        new_nfa
                    },
					Quantifier::Range(min, max) => {
						let base = stack.pop().expect("Error applying Range");
                        let (new_nfa, new_id) = range(base, min, max);
                        state_id = new_id;
                        new_nfa
					}
                },
                Operator::Concatenation | Operator::TrailingContent => {
                    let (left, right) = pop_last_two(&mut stack);
                    concatenate(left, right)
                }
                Operator::Or => {
                    let (left, right) = pop_last_two(&mut stack);
                    alternate(left, right)
                }
                _ => todo!(),
            },
        };

        dbg!(&nfa);
        stack.push(nfa);
    }

    todo!(); // Return the final result or handle empty stack
}
