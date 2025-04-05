use super::Operator as Operators;
use super::Operator::*;
use super::Token;
use super::Token::Operator;
use std::collections::VecDeque;

fn has_higher_precedence(current: &Operators, stack_top: &Operators) -> bool {
    let current_precedence = match current {
        Quantifier(_) => 4,
        Concatenation => 3,
        Or => 2,
        TrailingContent => 1,
        OpenGroup => 0,
        _ => panic!("Erreur dans le code"),
    };

    let stack_top_precedence = match stack_top {
        Quantifier(_) => 4,
        Concatenation => 3,
        Or => 2,
        TrailingContent => 1,
        OpenGroup => 0,
        _ => panic!("Erreur dans le code"),
    };

    return current_precedence - stack_top_precedence > 0;
}

pub fn to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut operator_stack: VecDeque<Operators> = VecDeque::new();
    let mut token_iter = tokens.iter();

    while let Some(token) = token_iter.next() {
        match *token {
            Token::Char(c) => output.push(Token::Char(c)),

            Operator(OpenGroup) => {
                operator_stack.push_front(OpenGroup);
            }

            Operator(CloseGroup) => loop {
                if let Some(top_operator) = operator_stack.pop_front() {
                    if top_operator == OpenGroup {
                        break;
                    }
                    output.push(Operator(top_operator));
                } else {
                    panic!("Parenthèse ouvrante manquante");
                }
            },

            Operator(current_op) => {
                while let Some(top_op) = operator_stack.front() {
                    if has_higher_precedence(&current_op, top_op) {
                        break;
                    }
                    output.push(Operator(*top_op));
                    operator_stack.pop_front();
                }
                operator_stack.push_front(current_op);
            }
        }
    }

    while let Some(remaining_op) = operator_stack.pop_front() {
        if remaining_op == OpenGroup {
            panic!("Parenthèse fermante manquante");
        }
        output.push(Operator(remaining_op));
    }

    return output;
}
