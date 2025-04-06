use super::Operator as Operators;
use super::Operator::*;
use super::Token;
use super::Token::Operator;
use std::collections::VecDeque;

fn precedence(op: &Operators) -> u8 {
    match op {
        Quantifier(_) => 4,
        Concatenation => 3,
        Or => 2,
        TrailingContent => 1,
        OpenParen => 0,
        _ => panic!("Opérateur non reconnu"),
    }
}

fn has_higher_precedence(current: &Operators, stack_top: &Operators) -> bool {
    return precedence(current) > precedence(stack_top);
}

pub fn to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut operator_stack: VecDeque<Operators> = VecDeque::new();
    let mut token_iter = tokens.iter();

    while let Some(token) = token_iter.next() {
        match *token {
            Token::Char(c) => output.push(Token::Char(c)),

            Operator(OpenParen) => {
                operator_stack.push_front(OpenParen);
            }

            Operator(CloseParen) => loop {
                if let Some(top_operator) = operator_stack.pop_front() {
                    if top_operator == OpenParen {
                        break;
                    }
                    output.push(Operator(top_operator));
                } else {
                    panic!("Parenthèse ouvrante manquante");
                }
            },

            Operator(current_op) => {
                while let Some(&top_op) = operator_stack.front() {
                    if has_higher_precedence(&current_op, &top_op) {
                        break;
                    }
                    operator_stack.pop_front();
                    output.push(Operator(top_op));
                }
                operator_stack.push_front(current_op);
            }
        }
    }

    while let Some(remaining_op) = operator_stack.pop_front() {
        if remaining_op == OpenParen {
            panic!("Parenthèse fermante manquante");
        }
        output.push(Operator(remaining_op));
    }

    return output;
}
