use super::Operator as Operators;
use super::Operator::*;
use super::Token;
use super::Token::Operator;
use std::collections::VecDeque;

fn precedence_check(first: &Operators, second: &Operators) -> bool {
    let first_precedence = match first {
        Quantifier(_) => 4,
        Concatenation => 3,
        Or => 2,
        TrailingContent => 1,
        OpenGroup => 0,
        _ => panic!("Error dans le code"),
    };

    let second_precedence = match second {
        Quantifier(_) => 4,
        Concatenation => 3,
        Or => 2,
        TrailingContent => 1,
        OpenGroup => 0,
        _ => panic!("Error dans le code"),
    };

    return first_precedence - second_precedence > 0;
}

pub fn postfix_notation(tokens: Vec<Token>) -> Vec<Token> {
    let mut dest: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut operator_stack: VecDeque<Operators> = VecDeque::new();
    let mut token_it = tokens.iter();

    while let Some(token) = token_it.next() {
		dbg!(&token);
        match *token {
            Token::Char(c) => dest.push(Token::Char(c)),
            Operator(OpenGroup) => {
                operator_stack.push_front(OpenGroup);
            }
            Operator(CloseGroup) => loop {
                if let Some(front) = operator_stack.pop_front() {
                    if front == OpenGroup {
                        break;
                    }
                    dest.push(Operator(front));
                } else {
                    panic!("Unopen Group");
                }
            },
            Operator(op) => {
                if let Some(front) = operator_stack.front() {
                    if precedence_check(&op, front) == false {
                        dest.push(Operator(*front));
                        operator_stack.pop_front();
                    }
                    operator_stack.push_front(op);
                } else {
                    operator_stack.push_front(op);
                }
            }
        }
    }

    while let Some(operator) = operator_stack.pop_front() {
        if operator == OpenGroup {
            panic!("Unclosed quotes");
        }
        dest.push(Operator(operator));
    }
    return dest;
}
