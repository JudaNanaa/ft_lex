use super::Operator::{CloseParen, Concatenation, OpenParen, Quantifier};
use super::Token;
use super::Token::Operator;

fn need_concatenate(token: &Token, next: &Token) -> bool {
    match token {
        Token::Char(_) => matches!(next, Token::Char(_) | Operator(OpenParen)),
        Operator(CloseParen) => matches!(next, Token::Char(_) | Operator(OpenParen)),
        Operator(Quantifier(_)) => matches!(next, Token::Char(_) | Operator(OpenParen)),
        Operator(_) => false,
    }
}

pub fn add_concatenation_token(tokens: &[Token]) -> Vec<Token> {
    let mut dest = Vec::with_capacity(tokens.len() * 2);
    let mut token_it = tokens.iter().peekable();

    while let Some(token) = token_it.next() {
        dest.push(*token);
        if let Some(next) = token_it.peek() {
            if need_concatenate(token, next) {
                dest.push(Operator(Concatenation));
            }
        }
    }
    dest
}

// --------------------------- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::{Operator, Quantifier};

    fn str_tokens(tokens: &[Token]) -> String {
        tokens
            .iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn test_simple_concatenation_between_chars() {
        let input = vec![Token::Char('a'), Token::Char('b')];
        let result = add_concatenation_token(&input);

        let expected = vec![
            Token::Char('a'),
            Operator(Operator::Concatenation),
            Token::Char('b'),
        ];

        assert_eq!(result, expected, "{}", str_tokens(&result));
    }

    #[test]
    fn test_char_followed_by_group() {
        let input = vec![Token::Char('a'), Operator(Operator::OpenParen)];
        let result = add_concatenation_token(&input);

        let expected = vec![
            Token::Char('a'),
            Operator(Operator::Concatenation),
            Operator(Operator::OpenParen),
        ];

        assert_eq!(result, expected, "{}", str_tokens(&result));
    }

    #[test]
    fn test_group_followed_by_char() {
        let input = vec![Operator(Operator::CloseParen), Token::Char('a')];
        let result = add_concatenation_token(&input);

        let expected = vec![
            Operator(Operator::CloseParen),
            Operator(Operator::Concatenation),
            Token::Char('a'),
        ];

        assert_eq!(result, expected, "{}", str_tokens(&result));
    }

    #[test]
    fn test_quantifier_followed_by_group() {
        let input = vec![
            Token::Char('a'),
            Operator(Operator::Quantifier(Quantifier::AtLeast(0))),
            Operator(Operator::OpenParen),
        ];
        let result = add_concatenation_token(&input);

        let expected = vec![
            Token::Char('a'),
            Operator(Operator::Quantifier(Quantifier::AtLeast(0))),
            Operator(Operator::Concatenation),
            Operator(Operator::OpenParen),
        ];

        assert_eq!(result, expected, "{}", str_tokens(&result));
    }

    #[test]
    fn test_no_concatenation_needed() {
        let input = vec![
            Operator(Operator::OpenParen),
            Token::Char('a'),
            Operator(Operator::Or),
            Token::Char('b'),
            Operator(Operator::CloseParen),
        ];
        let result = add_concatenation_token(&input);

        let expected = vec![
            Operator(Operator::OpenParen),
            Token::Char('a'),
            Operator(Operator::Or),
            Token::Char('b'),
            Operator(Operator::CloseParen),
        ];

        assert_eq!(result, expected, "{}", str_tokens(&result));
    }

    #[test]
    fn test_long_expression_with_multiple_concatenations() {
        let input = vec![
            Token::Char('a'),
            Token::Char('b'),
            Operator(Operator::OpenParen),
            Token::Char('c'),
            Operator(Operator::CloseParen),
        ];

        let result = add_concatenation_token(&input);

        let expected = vec![
            Token::Char('a'),
            Operator(Operator::Concatenation),
            Token::Char('b'),
            Operator(Operator::Concatenation),
            Operator(Operator::OpenParen),
            Token::Char('c'),
            Operator(Operator::CloseParen),
        ];

        assert_eq!(result, expected, "{}", str_tokens(&result));
    }
}
