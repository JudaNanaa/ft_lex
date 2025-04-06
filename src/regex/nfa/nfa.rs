use crate::regex::{
    nfa::{
        at_least::at_least, concatenate::concatenate, from_char::from_char, or::or, range::range,
        repeat_exact::repeat_exact, utils::pop_last_two,
    },
    Operator, Quantifier, Token, NFA,
};

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
                    }
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
                    or(left, right)
                }
                _ => todo!(),
            },
        };

        dbg!(&nfa);
        stack.push(nfa);
    }

    todo!(); // Return the final result or handle empty stack
}
