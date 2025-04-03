use std::collections::HashSet;

use crate::regex::Token;
use crate::regex::Operator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(usize);

#[derive(Debug, Clone)]
pub struct Transition {
    from: State,
    to: State,
    input: Option<char>,
}

#[derive(Debug)]
pub struct Automaton {
    transitions: Vec<Transition>,
    initial_state: State,
    final_states: HashSet<State>,
}

fn create_automata_char(char: char) -> Automaton {
    let q0 = State(0);
    let q1 = State(1);

    let transition = Transition {
        from: q0,
        to: q1,
        input: Some(char),
    };

    let mut final_s = HashSet::new();
    final_s.insert(q1);

    return Automaton {
        transitions: vec![transition],
        initial_state: q0,
        final_states: final_s,
    };
}

fn concatenation_two_automata(left: Automaton, right: Automaton) -> Automaton {
	
	todo!();
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

// a
pub fn create_nfa(tokens: &Vec<Token>) -> Automaton {
    let mut stack: Vec<Automaton> = Vec::new();
    // need stack
    for token in tokens {
        let automaton = match *token {
            Token::Char(c) => create_automata_char(c),
            Token::Operator(op) => match op {
				Operator::Concatenation => {
					let (left, right) = pop_two_last_element_stack(&mut stack);
					concatenation_two_automata(left, right)
				},
                _ => todo!(),
            },
        };
		println!("automaton = {:#?}", automaton);
		stack.push(automaton);
    }
    dbg!(stack);
    // if char -> automata du char
    // if | -> link les deux last de la stack en mode or
    // if concat -> concatener les deux last de la stack
    // if quantifier -> faire le quantifier sur le dernier de la stack

    todo!();
    // return stack.pop().unwrap();
}
